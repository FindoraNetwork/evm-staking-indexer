use crate::db::Storage;
use crate::error::Result;
use crate::{RewardContract, StakingContract, DEFAULT_INTERVAL};
use crossbeam::channel::bounded;
use ethers::prelude::{Http, Provider};
use ethers::providers::Middleware;
use ethers::types::Address;
use ethers::utils::hex::encode_prefixed;
use log::{error, info};

use sqlx::types::BigDecimal;
use std::str::FromStr;

use sqlx::types::chrono::{DateTime, NaiveDateTime};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub struct RpcCaller {
    pub retries: usize,
    pub provider: Provider<Http>,
    pub staking: StakingContract<Provider<Http>>,
    pub reward: RewardContract<Provider<Http>>,
    pub storage: Storage,
}

#[derive(Default)]
pub struct ContractValidator {
    block_num: i64,
    pubkey: String,
    pubkey_type: i32,
    rate: BigDecimal,
    staker: String,
    power: BigDecimal,
    total_unbound_amount: BigDecimal,
    punish_rate: BigDecimal,
    begin_block: i64,
    active: bool,
    jailed: bool,
    unjail_datetime: NaiveDateTime,
    should_vote: i32,
    voted: i32,
}

#[derive(Debug)]
pub struct Updater {
    caller: Arc<RpcCaller>,
}

impl Updater {
    pub fn new(
        retries: usize,
        provider: Provider<Http>,
        staking: StakingContract<Provider<Http>>,
        reward: RewardContract<Provider<Http>>,
        storage: Storage,
    ) -> Self {
        let caller = RpcCaller {
            retries,
            provider,
            staking,
            reward,
            storage,
        };

        Self {
            caller: Arc::new(caller),
        }
    }

    pub async fn update_validators(&self, validators: Vec<String>) -> Result<u64> {
        let block_num = self.caller.provider.get_block_number().await?.as_u64();

        let count = validators.len();
        let (sender, receiver) = bounded(count);
        let caller_cloned = self.caller.clone();

        let producer_handle = tokio::task::spawn_blocking(move || {
            for vaddr in validators {
                let addr = Address::from_str(&vaddr).unwrap();
                let fut = update_validator_task(caller_cloned.clone(), block_num, addr);
                sender.send(Some(fut)).unwrap();
            }

            for _ in 0..count {
                sender.send(None).unwrap();
            }
        });

        let consumer_handles: Vec<_> = (0..count)
            .map(move |_| {
                let r = receiver.clone();
                tokio::spawn(async move {
                    while let Ok(Some(fut)) = r.recv() {
                        let _ = fut.await;
                    }
                })
            })
            .collect();

        for h in consumer_handles {
            h.await?;
        }
        producer_handle.await?;

        Ok(block_num)
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let validators = self.caller.storage.get_validator_list().await?;
            if validators.len() > 0 {
                match self.update_validators(validators).await {
                    Ok(block_num) => {
                        info!("Update validators at block {} complete", block_num);
                    }
                    Err(e) => {
                        error!("Update validators error: {:?}", e);
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(DEFAULT_INTERVAL)).await
        }
    }
}

async fn update_validator_task(
    caller: Arc<RpcCaller>,
    block_num: u64,
    vaddr: Address,
) -> Result<()> {
    let mut validator = ContractValidator::default();
    validator.block_num = block_num as i64;
    //    struct ValidatorData {
    //         bytes publicKey;
    //         PublicKeyType ty;
    //         uint256 rate;
    //         address staker;
    //         uint256 power;
    //         uint256 totalUnboundAmount;
    //         uint256 punishRate;
    //         uint256 beginBlock;
    //     }
    match caller.staking.validators(vaddr).call().await {
        Ok(vdata) => {
            validator.pubkey = vdata.0.to_string();
            validator.pubkey_type = vdata.1 as i32;
            validator.rate = BigDecimal::from_str(&vdata.2.to_string()).unwrap_or_default();
            validator.staker = encode_prefixed(&vdata.3.to_string());
            validator.power = BigDecimal::from_str(&vdata.4.to_string()).unwrap_or_default();
            validator.total_unbound_amount =
                BigDecimal::from_str(&vdata.5.to_string()).unwrap_or_default();
            validator.punish_rate = BigDecimal::from_str(&vdata.6.to_string()).unwrap_or_default();
            validator.begin_block = vdata.7.as_u64() as i64;
        }
        Err(e) => {
            error!(" Get data of validator {:?} error: {:?}", vaddr, e)
        }
    }

    //  struct ValidatorStatus {
    //         uint256 heapIndexOff1;
    //         bool isActive;
    //         bool jailed;
    //         uint64 unjailDatetime;
    //         uint16 shouldVote;
    //         uint16 voted;
    //  }
    match caller.staking.validator_status(vaddr).call().await {
        Ok(vstatus) => {
            validator.active = vstatus.1;
            validator.jailed = vstatus.2;
            let tm = DateTime::from_timestamp(vstatus.3 as i64, 0)
                .unwrap()
                .naive_utc();
            validator.unjail_datetime = tm;
            validator.should_vote = vstatus.4 as i32;
            validator.voted = vstatus.5 as i32;
        }
        Err(e) => {
            error!("Get status of validator {:?} error: {:?}", vaddr, e)
        }
    }

    caller
        .storage
        .insert_validator(
            validator.block_num,
            &encode_prefixed(vaddr.as_bytes()),
            &validator.pubkey,
            validator.pubkey_type,
            validator.rate,
            &validator.staker,
            validator.power,
            validator.total_unbound_amount,
            validator.punish_rate,
            validator.begin_block,
            validator.active,
            validator.jailed,
            validator.unjail_datetime,
            validator.should_vote,
            validator.voted,
        )
        .await?;

    Ok(())
}
