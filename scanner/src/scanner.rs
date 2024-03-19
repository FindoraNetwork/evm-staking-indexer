use crate::db::Storage;
use crate::error::Result;
use crate::error::ScannerError;
use crate::tdrpc::TendermintRpc;
use crate::types::{CallData, ContractOp, CreateData, RpcBlock, TxCallLog};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use crossbeam::channel::bounded;
use ethabi::ethereum_types::H256;
use ethabi::{Event, EventParam, Hash, Log, ParamType, RawLog};
use ethers::utils::hex;
use log::{error, info};
use serde_json::Value;
use sha2::Digest;
use sqlx::types::BigDecimal;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

// "evm:"
const EVM_TX_TAG: [u8; 4] = [0x65, 0x76, 0x6d, 0x3a];
const DEFAULT_RPC_TIMEOUT: u64 = 30; // 30s

// Stake(
//  address indexed validator,
//  bytes publicKey,
//  PublicKeyType ty,
//  address indexed staker,
//  uint256 amount,
//  string memo,
//  uint256 rate);
const EVENT_STAKE_TOPIC: &str =
    "0x498a21473058cc6f2e2b7da5292de11377e3987136d6a96b5c2b170952fcf5c4";

// Delegation(
//  address indexed validator,
//  address indexed delegator,
//  uint256 amount);
const EVENT_DELEGATION_TOPIC: &str =
    "0x96eafeca8c3c21ab2fa4a636b93ba20c9e22e3d222d92c6530fedc29a53671ee";

// Undelegation(
//  uint256 index,
//  address indexed validator,
//  address indexed delegator,
//  uint256 unlockTime,
//  uint256 amount,
//  uint8 operationType);
const EVENT_UNDELEGATION_TOPIC: &str =
    "0x248cda0b34d17f8cf3b592aed07dd22da583ae74483d40a96004f53847b71954";

// Epoch(uint256 epoch);
const EVENT_EPOCH_TOPIC: &str =
    "0xc1d4931e10652da8ab23604510531810d2eebfcd33a81ba4946d702ce8057b64";

// Jailed(
//  address indexed validator,
//  bool jailed);
const EVENT_JAILED_TOPIC: &str =
    "0xb9b790eb0e7064670ac68f8299688933c4c510bc09f49d8c12b74c7d4fdde56f";

// Punish(
//  address[] voted,
//  address[] unvoted,
//  address[] byztine);
const EVENT_PUNISH_TOPIC: &str =
    "0x0b76ecf3bf29ec85175361a38c68eb2d1bb7de232f25bdb17924cb3c2a5bc685";

// UpdateValidator(
//  address indexed validator,
//  string memo,
//  uint256 rate);
const EVENT_UPDATE_VALIDATOR_TOPIC: &str =
    "0xcb3de9afda95cbc521d863fe4ec8bad7569847876bdda3c69aa406c14bd9486b";

// event Proposer(address proposer);
const EVENT_PROPOSER_TOPIC: &str =
    "0xa990523a550e65422b3b987dda53586fadb4067c5e34841901d2f74a5c81e4ad";

// CoinbaseMint(
//  address indexed validator,
//  address indexed delegator,
//  bytes publicKey,
//  uint256 amount);
const EVENT_COINBASE_MINT: &str =
    "0xb2cf206b70e745484dd39dc6b8e6166ce07246bd00baa4bd059f15733b2130e9";

pub struct RpcCaller {
    pub retries: usize,
    pub rpc: TendermintRpc,
    pub storage: Storage,
    pub threads: usize,
}

impl RpcCaller {
    pub async fn get_block_retried(&self, height: u64) -> Result<()> {
        let mut retries: usize = 0;
        let mut err = ScannerError::BlockNotFound(height);

        while retries < self.retries {
            match self.rpc.get_block_by_height(height).await {
                Ok(block) => {
                    self.process_block(block).await?;
                    return Ok(());
                }
                Err(ScannerError::BlockNotFound(h)) => return Err(ScannerError::BlockNotFound(h)),
                Err(e) => {
                    error!("Get block {}: {:?}", height, e);
                    err = e;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }

            retries += 1;
        }

        Err(err)
    }
    async fn parse_event_log(&self, log: TxCallLog, event: Event) -> Result<Log> {
        let mut topics: Vec<H256> = vec![];
        for t in log.topics {
            if let Ok(h) = Hash::from_str(&t) {
                topics.push(h);
            }
        }

        let raw_log = RawLog {
            topics,
            data: log.data,
        };
        let log_parsed = event.parse_log(raw_log)?;

        Ok(log_parsed)
    }

    async fn process_contract_result(
        &self,
        tx_hash: &str,
        block_num: i64,
        contract_result: ContractOp,
    ) -> Result<()> {
        let logs = match contract_result {
            ContractOp::Create(create_data) => create_data.create.logs,
            ContractOp::Call(call_data) => call_data.call.logs,
        };

        for log in logs {
            match log.topics[0].as_str() {
                EVENT_STAKE_TOPIC => {
                    // Stake(
                    //  address indexed validator,
                    //  bytes public_key,
                    //  PublicKeyType ty,
                    //  address indexed staker,
                    //  uint256 amount,
                    //  string memo,
                    //  uint256 rate);
                    let params: Vec<EventParam> = vec![
                        EventParam {
                            name: "validator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "public_key".to_string(),
                            kind: ParamType::Bytes,
                            indexed: false,
                        },
                        EventParam {
                            name: "ty".to_string(),
                            kind: ParamType::Uint(8),
                            indexed: false,
                        },
                        EventParam {
                            name: "staker".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "amount".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                        EventParam {
                            name: "memo".to_string(),
                            kind: ParamType::String,
                            indexed: false,
                        },
                        EventParam {
                            name: "rate".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                    ];

                    let e = Event {
                        name: "Stake".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    let amount = BigDecimal::from_str(
                        &log_parsed.params[4]
                            .value
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128()
                            .to_string(),
                    )
                    .unwrap_or(BigDecimal::from(0));
                    let rate = BigDecimal::from_str(
                        &log_parsed.params[6]
                            .value
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128()
                            .to_string(),
                    )
                    .unwrap_or(BigDecimal::from(0));

                    self.storage
                        .upsert_stake(
                            &tx_hash,
                            block_num,
                            &log_parsed.params[0].value.to_string(),
                            &log_parsed.params[1].value.to_string(),
                            log_parsed.params[2]
                                .value
                                .clone()
                                .into_uint()
                                .unwrap()
                                .as_u64() as i32,
                            &log_parsed.params[3].value.to_string(),
                            amount,
                            &log_parsed.params[5].value.to_string(),
                            rate,
                        )
                        .await?
                }
                EVENT_DELEGATION_TOPIC => {
                    // Delegation(
                    //  address indexed validator,
                    //  address indexed delegator,
                    //  uint256 amount);
                    let params: Vec<EventParam> = vec![
                        EventParam {
                            name: "validator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "delegator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "amount".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                    ];
                    let e = Event {
                        name: "Delegation".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    let amount = BigDecimal::from_str(
                        &log_parsed.params[2]
                            .value
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128()
                            .to_string(),
                    )
                    .unwrap_or(BigDecimal::from(0));

                    self.storage
                        .upsert_delegation(
                            &tx_hash,
                            block_num,
                            &log_parsed.params[0].value.to_string(),
                            &log_parsed.params[1].value.to_string(),
                            amount,
                        )
                        .await?
                }
                EVENT_UNDELEGATION_TOPIC => {
                    // Undelegation(
                    //  uint256 index,
                    //  address indexed validator,
                    //  address indexed delegator,
                    //  uint256 unlockTime,
                    //  uint256 amount,
                    //  uint8 operationType);
                    let params: Vec<EventParam> = vec![
                        EventParam {
                            name: "index".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                        EventParam {
                            name: "validator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "delegator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "unlockTime".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                        EventParam {
                            name: "amount".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                        EventParam {
                            name: "operationType".to_string(),
                            kind: ParamType::Uint(8),
                            indexed: false,
                        },
                    ];
                    let e = Event {
                        name: "Undelegation".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    let amount = BigDecimal::from_str(
                        &log_parsed.params[4]
                            .value
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128()
                            .to_string(),
                    )
                    .unwrap_or(BigDecimal::from(0));

                    self.storage
                        .upsert_undelegation(
                            &tx_hash,
                            block_num,
                            log_parsed.params[0]
                                .value
                                .clone()
                                .into_uint()
                                .unwrap()
                                .as_u64() as i32,
                            &log_parsed.params[1].value.to_string(),
                            &log_parsed.params[2].value.to_string(),
                            log_parsed.params[3]
                                .value
                                .clone()
                                .into_uint()
                                .unwrap()
                                .as_u64() as i64,
                            amount,
                            log_parsed.params[5]
                                .value
                                .clone()
                                .into_uint()
                                .unwrap()
                                .as_u64() as i32,
                        )
                        .await?
                }
                EVENT_PROPOSER_TOPIC => {
                    // event Proposer(address proposer);
                    let params: Vec<EventParam> = vec![EventParam {
                        name: "proposer".to_string(),
                        kind: ParamType::Address,
                        indexed: false,
                    }];
                    let e = Event {
                        name: "Proposer".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    self.storage
                        .upsert_proposer(
                            &tx_hash,
                            block_num,
                            &log_parsed.params[0].value.to_string(),
                        )
                        .await?
                }
                EVENT_EPOCH_TOPIC => {
                    // Epoch(uint256 epoch);
                    let params: Vec<EventParam> = vec![EventParam {
                        name: "epoch".to_string(),
                        kind: ParamType::Uint(256),
                        indexed: false,
                    }];
                    let e = Event {
                        name: "Epoch".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    self.storage
                        .upsert_epoch(
                            &tx_hash,
                            block_num,
                            log_parsed.params[0]
                                .value
                                .clone()
                                .into_uint()
                                .unwrap()
                                .as_u64() as i64,
                        )
                        .await?
                }
                EVENT_JAILED_TOPIC => {
                    // Jailed(
                    //  address indexed validator,
                    //  bool jailed);
                    let params: Vec<EventParam> = vec![
                        EventParam {
                            name: "validator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "jailed".to_string(),
                            kind: ParamType::Bool,
                            indexed: false,
                        },
                    ];
                    let e = Event {
                        name: "Jailed".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    self.storage
                        .upsert_jailed(
                            &tx_hash,
                            block_num,
                            &log_parsed.params[0].value.to_string(),
                            log_parsed.params[1].value.clone().into_bool().unwrap(),
                        )
                        .await?
                }
                EVENT_PUNISH_TOPIC => {
                    // Punish(
                    //  address[] voted,
                    //  address[] unvoted,
                    //  address[] byztine);
                    let params: Vec<EventParam> = vec![
                        EventParam {
                            name: "voted".to_string(),
                            kind: ParamType::Array(Box::new(ParamType::Address)),
                            indexed: false,
                        },
                        EventParam {
                            name: "unvoted".to_string(),
                            kind: ParamType::Array(Box::new(ParamType::Address)),
                            indexed: false,
                        },
                        EventParam {
                            name: "byztine".to_string(),
                            kind: ParamType::Array(Box::new(ParamType::Address)),
                            indexed: false,
                        },
                    ];
                    let e = Event {
                        name: "Punish".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;

                    let voted: Value =
                        serde_json::from_str(&log_parsed.params[0].value.to_string())?;
                    let unvoted: Value =
                        serde_json::from_str(&log_parsed.params[1].value.to_string())?;
                    let byztine: Value =
                        serde_json::from_str(&log_parsed.params[2].value.to_string())?;
                    self.storage
                        .upsert_punish(&tx_hash, block_num, voted, unvoted, byztine)
                        .await?
                }
                EVENT_UPDATE_VALIDATOR_TOPIC => {
                    // UpdateValidator(
                    //  address indexed validator,
                    //  string memo,
                    //  uint256 rate);
                    let params: Vec<EventParam> = vec![
                        EventParam {
                            name: "validator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "memo".to_string(),
                            kind: ParamType::String,
                            indexed: false,
                        },
                        EventParam {
                            name: "rate".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                    ];
                    let e = Event {
                        name: "UpdateValidator".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    let rate = BigDecimal::from_str(
                        &log_parsed.params[2]
                            .value
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128()
                            .to_string(),
                    )
                    .unwrap_or(BigDecimal::from(0));
                    self.storage
                        .upsert_update_validator(
                            &tx_hash,
                            block_num,
                            &log_parsed.params[0].value.to_string(),
                            &log_parsed.params[1].value.to_string(),
                            rate,
                        )
                        .await?
                }
                EVENT_COINBASE_MINT => {
                    // CoinbaseMint(
                    //  address indexed validator,
                    //  address indexed delegator,
                    //  bytes publicKey,
                    //  uint256 amount);
                    let params: Vec<EventParam> = vec![
                        EventParam {
                            name: "validator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "delegator".to_string(),
                            kind: ParamType::Address,
                            indexed: true,
                        },
                        EventParam {
                            name: "publicKey".to_string(),
                            kind: ParamType::Bytes,
                            indexed: false,
                        },
                        EventParam {
                            name: "amount".to_string(),
                            kind: ParamType::Uint(256),
                            indexed: false,
                        },
                    ];

                    let e = Event {
                        name: "CoinbaseMint".to_string(),
                        inputs: params,
                        anonymous: false,
                    };
                    let log_parsed = self.parse_event_log(log, e).await?;
                    let amount = BigDecimal::from_str(
                        &log_parsed.params[3]
                            .value
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128()
                            .to_string(),
                    )
                    .unwrap_or(BigDecimal::from(0));

                    self.storage
                        .upsert_coinbase_mint(
                            &tx_hash,
                            block_num,
                            &log_parsed.params[0].value.to_string(),
                            &log_parsed.params[1].value.to_string(),
                            &log_parsed.params[2].value.to_string(),
                            amount,
                        )
                        .await?
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn process_block(&self, block: RpcBlock) -> Result<()> {
        let block_num = block.block.header.height.parse::<i64>()?;
        if let Some(txs) = block.block.data.txs {
            for tx in txs {
                let tx_raw = BASE64_STANDARD.decode(&tx)?;
                if !EVM_TX_TAG.eq(&tx_raw[..4]) {
                    // not evm tx
                    continue;
                }
                let tx_hash = hex::encode(sha2::Sha256::digest(&tx_raw));
                let tx = self.rpc.get_tx_by_hash(tx_hash.as_str()).await?;
                if tx.tx_result.data.is_none() {
                    // no logs
                    continue;
                }
                let tx_result = BASE64_STANDARD.decode(tx.tx_result.data.unwrap())?;
                let contract_result =
                    if let Ok(call_data) = serde_json::from_slice::<CallData>(&tx_result) {
                        ContractOp::Call(call_data)
                    } else {
                        ContractOp::Create(serde_json::from_slice::<CreateData>(&tx_result)?)
                    };

                self.process_contract_result(&tx_hash, block_num, contract_result)
                    .await?
            }
        };

        Ok(())
    }
}

pub struct Scanner {
    caller: Arc<RpcCaller>,
}

impl Scanner {
    pub fn new(retries: usize, threads: usize, url: Url, storage: Storage) -> Result<Self> {
        let caller = RpcCaller {
            retries,
            rpc: TendermintRpc::new(url, Duration::from_secs(DEFAULT_RPC_TIMEOUT)),
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
