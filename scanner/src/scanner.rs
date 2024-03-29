use crate::db::Storage;
use crate::error::Result;
use crate::error::ScannerError;
use crossbeam::channel::bounded;
use ethers::contract::{parse_log, EthEvent};
use ethers::prelude::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::types::Bytes;
use ethers::types::U256;
use ethers::utils::hex::encode_prefixed;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::DateTime;
use sqlx::types::BigDecimal;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

const EVENT_STAKE_TOPIC: &str =
    "0x498a21473058cc6f2e2b7da5292de11377e3987136d6a96b5c2b170952fcf5c4";

const EVENT_DELEGATION_TOPIC: &str =
    "0x96eafeca8c3c21ab2fa4a636b93ba20c9e22e3d222d92c6530fedc29a53671ee";

const EVENT_UNDELEGATION_TOPIC: &str =
    "0x248cda0b34d17f8cf3b592aed07dd22da583ae74483d40a96004f53847b71954";

const EVENT_JAILED_TOPIC: &str =
    "0xb9b790eb0e7064670ac68f8299688933c4c510bc09f49d8c12b74c7d4fdde56f";

const EVENT_PUNISH_TOPIC: &str =
    "0x0b76ecf3bf29ec85175361a38c68eb2d1bb7de232f25bdb17924cb3c2a5bc685";

const EVENT_UPDATE_VALIDATOR_TOPIC: &str =
    "0xcb3de9afda95cbc521d863fe4ec8bad7569847876bdda3c69aa406c14bd9486b";

const EVENT_COINBASE_MINT_TOPIC: &str =
    "0xb2cf206b70e745484dd39dc6b8e6166ce07246bd00baa4bd059f15733b2130e9";

pub struct RpcCaller {
    pub retries: usize,
    pub provider: Provider<Http>,
    pub storage: Storage,
    pub threads: usize,
}

// Stake(address indexed validator,bytes publicKey, PublicKeyType ty, address indexed staker, uint256 amount, string memo, uint256 rate);
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
#[ethevent(abi = "Stake(address,bytes,uint8,address,uint256,string,uint256)")]
pub struct EventStake {
    #[ethevent(indexed)]
    pub validator: Address,
    #[ethevent(name = "publicKey")]
    pub public_key: Bytes,
    pub ty: u8,
    #[ethevent(indexed)]
    pub staker: Address,
    pub amount: U256,
    pub memo: String,
    pub rate: U256,
}

// Delegation( address indexed validator,address indexed delegator, uint256 amount);
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
#[ethevent(abi = "Delegation(address,address,uint256)")]
pub struct EventDelegation {
    #[ethevent(indexed)]
    pub validator: Address,
    #[ethevent(indexed)]
    pub delegator: Address,
    pub amount: U256,
}

// Undelegation(uint256 index, address indexed validator, address indexed delegator, uint256 unlockTime, uint256 amount, uint8 operationType);
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
#[ethevent(abi = "Undelegation(uint256,address,address,uint256,uint256,uint8)")]
pub struct EventUndelegation {
    pub index: U256,
    #[ethevent(indexed)]
    pub validator: Address,
    #[ethevent(indexed)]
    pub delegator: Address,
    #[ethevent(name = "unlockTime")]
    pub unlock_time: U256,
    pub amount: U256,
    #[ethevent(name = "operationType")]
    pub operation_type: u8,
}

// CoinbaseMint(address indexed validator, address indexed delegator, bytes publicKey, uint256 amount);
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
#[ethevent(abi = "CoinbaseMint(address,address,bytes,uint256)")]
pub struct EventCoinbaseMint {
    #[ethevent(indexed)]
    pub validator: Address,
    #[ethevent(indexed)]
    pub delegator: Address,
    #[ethevent(name = "publicKey")]
    pub public_key: Bytes,
    pub amount: U256,
}

// Jailed(address indexed validator,bool jailed);
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
#[ethevent(abi = "Jailed(address,bool)")]
pub struct EventJailed {
    #[ethevent(indexed)]
    pub validator: Address,
    pub jailed: bool,
}

// Punish(address[] voted,address[] unvoted,address[] byztine);
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
#[ethevent(abi = "Punish(address[],address[],address[])")]
pub struct EventPunish {
    pub voted: Vec<Address>,
    pub unvoted: Vec<Address>,
    pub byztine: Vec<Address>,
}

// UpdateValidator(address indexed validator, string memo,uint256 rate);
#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
#[ethevent(abi = "UpdateValidator(address,string,uint256)")]
pub struct EventUpdateValidator {
    #[ethevent(indexed)]
    pub validator: Address,
    pub memo: String,
    pub rate: U256,
}

impl RpcCaller {
    pub async fn get_block_retried(&self, height: u64) -> Result<()> {
        let block_opt = self.provider.get_block(height).await?;
        if block_opt.is_none() {
            return Err(ScannerError::BlockNotFound(height));
        }

        let block = block_opt.unwrap();
        let block_id = encode_prefixed(block.hash.unwrap().0);
        let tm = DateTime::from_timestamp(block.timestamp.as_u64() as i64, 0)
            .unwrap()
            .naive_utc();

        for tx_hash in block.transactions {
            debug!("Syncing tx receipt: {:?}", tx_hash);
            let tx_id = encode_prefixed(tx_hash.0);
            let receipt_opt = self.provider.get_transaction_receipt(tx_hash).await?;
            if receipt_opt.is_none() {
                error!("tx not found: {:?} at block {:?}", tx_id, height);
                continue;
            }
            let receipt = receipt_opt.unwrap();
            let receipt_val = serde_json::to_value(&receipt)?;
            self.storage
                .upsert_evm_receipt(
                    &tx_id,
                    &block_id,
                    height as i64,
                    &encode_prefixed(receipt.from.to_fixed_bytes()),
                    &encode_prefixed(receipt.to.unwrap_or_default().to_fixed_bytes()),
                    tm,
                    receipt_val,
                )
                .await?;
            for log in receipt.logs {
                match encode_prefixed(log.topics[0].as_bytes()).as_str() {
                    EVENT_STAKE_TOPIC => {
                        debug!("Stake:{:?}", encode_prefixed(log.topics[0]).to_string());
                        let stake: EventStake = parse_log(log.clone())?;
                        let mem_val: Value = serde_json::from_str(&stake.memo)?;
                        self.storage
                            .upsert_stake(
                                &tx_id,
                                &block_id,
                                height as i64,
                                tm,
                                &encode_prefixed(stake.validator.as_bytes()),
                                &stake.public_key.to_string(),
                                stake.ty as i32,
                                &encode_prefixed(stake.staker),
                                BigDecimal::from_str(&stake.amount.as_u128().to_string())
                                    .unwrap_or_default(),
                                mem_val,
                                BigDecimal::from_str(&stake.rate.as_u128().to_string())
                                    .unwrap_or_default(),
                            )
                            .await?
                    }
                    EVENT_DELEGATION_TOPIC => {
                        debug!("Delegate: {:?}", encode_prefixed(log.topics[0]).to_string());
                        let delegation: EventDelegation = parse_log(log.clone())?;
                        self.storage
                            .upsert_delegation(
                                &tx_id,
                                &block_id,
                                height as i64,
                                tm,
                                &encode_prefixed(delegation.validator.as_bytes()),
                                &encode_prefixed(delegation.delegator.as_bytes()),
                                BigDecimal::from_str(&delegation.amount.as_u128().to_string())
                                    .unwrap_or_default(),
                            )
                            .await?
                    }
                    EVENT_UNDELEGATION_TOPIC => {
                        debug!(
                            "Undelegate: {:?}",
                            encode_prefixed(log.topics[0]).to_string()
                        );
                        let undelegation: EventUndelegation = parse_log(log.clone())?;
                        self.storage
                            .upsert_undelegation(
                                &tx_id,
                                &block_id,
                                height as i64,
                                tm,
                                undelegation.index.as_u32() as i32,
                                &encode_prefixed(undelegation.validator.as_bytes()),
                                &encode_prefixed(undelegation.delegator.as_bytes()),
                                undelegation.unlock_time.as_u64() as i64,
                                BigDecimal::from_str(&undelegation.amount.as_u128().to_string())
                                    .unwrap_or_default(),
                                undelegation.operation_type as i32,
                            )
                            .await?
                    }
                    EVENT_COINBASE_MINT_TOPIC => {
                        debug!(
                            "[CoinbaseMint] {:?}",
                            encode_prefixed(log.topics[0]).to_string()
                        );
                        let coinbase_mint: EventCoinbaseMint = parse_log(log.clone())?;
                        self.storage
                            .upsert_coinbase_mint(
                                &tx_id,
                                &block_id,
                                height as i64,
                                tm,
                                &encode_prefixed(coinbase_mint.validator.as_bytes()),
                                &encode_prefixed(coinbase_mint.delegator.as_bytes()),
                                &coinbase_mint.public_key.to_string(),
                                BigDecimal::from_str(&coinbase_mint.amount.as_u128().to_string())
                                    .unwrap_or_default(),
                            )
                            .await?
                    }
                    EVENT_JAILED_TOPIC => {
                        debug!("[Jailed] {:?}", encode_prefixed(log.topics[0]).to_string());
                        let jailed: EventJailed = parse_log(log.clone())?;
                        self.storage
                            .upsert_jailed(
                                &tx_id,
                                &block_id,
                                height as i64,
                                tm,
                                &encode_prefixed(jailed.validator.as_bytes()),
                                jailed.jailed,
                            )
                            .await?
                    }
                    EVENT_UPDATE_VALIDATOR_TOPIC => {
                        debug!(
                            "[UpdateValidator] {:?}",
                            encode_prefixed(log.topics[0]).to_string()
                        );
                        let update_validator: EventUpdateValidator = parse_log(log.clone())?;
                        let memo_val = serde_json::to_value(update_validator.memo)?;

                        self.storage
                            .upsert_update_validator(
                                &tx_id,
                                &block_id,
                                height as i64,
                                tm,
                                &encode_prefixed(update_validator.validator.as_bytes()),
                                memo_val,
                                BigDecimal::from_str(&update_validator.rate.as_u128().to_string())
                                    .unwrap_or_default(),
                            )
                            .await?
                    }
                    EVENT_PUNISH_TOPIC => {
                        debug!("[Punish] {:?}", encode_prefixed(log.topics[0]).to_string());
                        let punish: EventPunish = parse_log(log.clone())?;
                        let voted_val = serde_json::to_value(punish.voted)?;
                        let unvoted_val = serde_json::to_value(punish.unvoted)?;
                        let byzantine_val = serde_json::to_value(punish.byztine)?;
                        self.storage
                            .upsert_punish(
                                &tx_id,
                                &block_id,
                                height as i64,
                                tm,
                                voted_val,
                                unvoted_val,
                                byzantine_val,
                            )
                            .await?
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

pub struct Scanner {
    caller: Arc<RpcCaller>,
}

impl Scanner {
    pub fn new(
        retries: usize,
        threads: usize,
        provider: Provider<Http>,
        storage: Storage,
    ) -> Result<Self> {
        let caller = RpcCaller {
            retries,
            provider,
            storage,
            threads,
        };

        Ok(Self {
            caller: Arc::new(caller),
        })
    }

    pub async fn single_scan(&self, height: u64) -> Result<()> {
        info!("Syncing block: {}", height);
        self.caller.get_block_retried(height).await?;
        self.caller.storage.upsert_tip(height as i64).await?;
        info!("Syncing block: {} complete", height);
        Ok(())
    }

    pub async fn range_scan(&self, start: u64, end: u64) -> Result<u64> {
        info!("Syncing [{},{}) ...", start, end);
        let concurrency = self.caller.threads; //how many spawned.
        let (sender, receiver) = bounded(concurrency);
        let last_height = Arc::new(AtomicU64::new(0));
        let succeed_cnt = Arc::new(AtomicU64::new(0));
        let caller_cloned = self.caller.clone();
        let last_height_cloned = last_height.clone();
        let succeed_cnt_cloned = succeed_cnt.clone();

        let producer_handle = tokio::task::spawn_blocking(move || {
            for h in start..end {
                let fut = task(
                    caller_cloned.clone(),
                    h,
                    last_height_cloned.clone(),
                    succeed_cnt_cloned.clone(),
                );

                sender.send(Some(fut)).unwrap();
            }

            for _ in 0..concurrency {
                sender.send(None).unwrap();
            }
        });

        let consumer_handles: Vec<_> = (0..concurrency)
            .map(move |_| {
                let r = receiver.clone();
                tokio::spawn(async move {
                    while let Ok(Some(fut)) = r.recv() {
                        fut.await;
                    }
                })
            })
            .collect();

        for h in consumer_handles {
            h.await?;
        }
        producer_handle.await?;
        info!("Syncing [{},{}) complete.", start, end);
        Ok(succeed_cnt.load(Ordering::Acquire))
    }

    pub async fn run(&self, start: u64, interval: Duration, single: bool) -> Result<()> {
        match single {
            true => {
                info!("Single syncing...");
                self.single_scan(start).await
            }
            false => {
                let mut height = start;
                let batch = (4 * self.caller.threads) as u64;
                info!("Fast syncing...");
                loop {
                    let cnt = self.range_scan(height, height + batch).await?;
                    if cnt == batch {
                        height += batch;
                    } else {
                        break;
                    }
                }
                info!("Fast syncing complete.");
                loop {
                    if let Ok(h) = self.caller.storage.get_tip().await {
                        height = h as u64 + 1;
                    }

                    match self.caller.get_block_retried(height).await {
                        Ok(_) => {
                            info!("Get block {} succeed", height);
                            self.caller.storage.upsert_tip(height as i64).await?;
                        }
                        Err(ScannerError::BlockNotFound(height)) => {
                            error!("Block {} not found", height)
                        }
                        Err(e) => {
                            error!("Get block {} error: {:?}", height, e);
                        }
                    }

                    tokio::time::sleep(interval).await;
                }
            }
        }
    }
}

async fn task(
    caller: Arc<RpcCaller>,
    height: u64,
    last_height: Arc<AtomicU64>,
    succeed_cnt: Arc<AtomicU64>,
) {
    match caller.get_block_retried(height).await {
        Ok(_) => {
            let h_old = last_height.load(Ordering::Acquire);
            if height > h_old {
                last_height.store(height, Ordering::Release);
                if let Err(e) = caller.storage.upsert_tip(height as i64).await {
                    error!("DB error: {:?}", e);
                }
            }
            succeed_cnt.fetch_add(1, Ordering::Release);
        }
        Err(ScannerError::BlockNotFound(h)) => error!("Block not found: {}", h),
        Err(e) => error!("Get block {} failed: {:?}", height, e),
    }
}
