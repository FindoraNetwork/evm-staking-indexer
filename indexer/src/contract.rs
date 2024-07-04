use crate::error::{IndexerError, Result};
use crate::types::{
    BoundResponse, DebtResponse, DelegatorSumResponse, RewardResponse, ValidatorDataResponse,
    ValidatorStatusResponse,
};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use ethers::types::H160;
use ethers::utils::hex;
use rand::Rng;
use redis::{Commands, Connection, RedisResult, SetExpiry, SetOptions};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::str::FromStr;
use std::sync::Arc;

const KEY_BOUND_PREFIX: &str = "E:BND";
const KEY_REWARD_PREFIX: &str = "E:RWD";

#[derive(Serialize, Deserialize)]
pub struct ValidatorStatusParams {
    pub address: String,
}

pub async fn get_validator_status(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorStatusParams>,
) -> Result<Json<ValidatorStatusResponse>> {
    let staking = state.staking.clone();
    let validator = H160::from_str(&params.address)?;

    match staking.validator_status(validator).call().await {
        Ok(data) => {
            let res = ValidatorStatusResponse {
                heap_index_off1: data.0.to_string(),
                is_active: data.1,
                jailed: data.2,
                unjail_datetime: data.3,
                should_vote: data.4,
                voted: data.5,
            };
            Ok(Json(res))
        }
        Err(e) => Err(IndexerError::IndexerCustom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorDataParams {
    pub address: String,
}

pub async fn get_validator_data(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorDataParams>,
) -> Result<Json<ValidatorDataResponse>> {
    let staking = state.staking.clone();
    let validator = H160::from_str(&params.address)?;

    match staking.validators(validator).call().await {
        Ok(data) => {
            let res = ValidatorDataResponse {
                public_key: data.0.to_string(),
                public_key_type: data.1,
                rate: data.2.to_string(),
                staker: hex::encode_prefixed(&data.3.as_bytes()),
                power: data.4.to_string(),
                total_unbound_amount: data.5.to_string(),
                begin_block: data.6.as_u64(),
            };
            Ok(Json(res))
        }
        Err(e) => Err(IndexerError::IndexerCustom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorBoundParams {
    pub validator: String,
    pub delegator: String,
}

pub async fn get_delegator_bound(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorBoundParams>,
) -> Result<Json<BoundResponse>> {
    let validator = H160::from_str(&params.validator)?;
    let delegator = H160::from_str(&params.delegator)?;
    let mut conn = state.redis.clone().get_connection()?;
    let key = format!("{}:{:?}:{:?}", KEY_BOUND_PREFIX, delegator, validator);
    let r: RedisResult<String> = conn.get(&key);
    match r {
        Ok(data) => {
            let resp: BoundResponse = serde_json::from_str(&data).unwrap();
            Ok(Json(resp))
        }
        Err(_) => {
            let staking = state.staking.clone();
            match staking.delegators(validator, delegator).call().await {
                Ok((bound, unbound)) => {
                    let resp = BoundResponse {
                        bound_amount: bound.to_string(),
                        unbound_amount: unbound.to_string(),
                    };
                    let data = serde_json::to_string(&resp).unwrap();
                    set_to_redis(&mut conn, &key, &data)?;
                    Ok(Json(resp))
                }
                Err(e) => return Err(IndexerError::IndexerCustom(e.to_string())),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorRewardParams {
    pub address: String,
}

pub async fn get_delegator_reward(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorRewardParams>,
) -> Result<Json<RewardResponse>> {
    let reward = state.reward.clone();
    let delegator = H160::from_str(&params.0.address)?;
    let mut conn = state.redis.clone().get_connection()?;
    let key = format!("{}:{:?}", KEY_REWARD_PREFIX, delegator);
    let r: RedisResult<String> = conn.get(&key);
    match r {
        Ok(data) => {
            let resp: RewardResponse = serde_json::from_str(&data).unwrap();
            Ok(Json(resp))
        }
        Err(_) => match reward.rewards(delegator).call().await {
            Ok(amount) => {
                let resp = RewardResponse {
                    reward: amount.to_string(),
                };
                let data = serde_json::to_string(&resp).unwrap();
                set_to_redis(&mut conn, &key, &data)?;
                Ok(Json(resp))
            }
            Err(e) => return Err(IndexerError::IndexerCustom(e.to_string())),
        },
    }
}

fn set_to_redis(conn: &mut Connection, key: &str, value: &str) -> Result<()> {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..10);
    let ms = 10 * 60 * 1000 + r * 1000; // 10 min + r min
    let opts = SetOptions::default()
        .get(true)
        .with_expiration(SetExpiry::PX(ms));
    conn.set_options(&key, value, opts)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorDebtParams {
    pub validator: String,
    pub delegator: String,
}

pub async fn get_delegator_debt(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorDebtParams>,
) -> Result<Json<DebtResponse>> {
    let reward = state.reward.clone();
    let validator = H160::from_str(&params.validator)?;
    let delegator = H160::from_str(&params.delegator)?;
    match reward.reward_debt(validator, delegator).call().await {
        Ok(amount) => Ok(Json(DebtResponse {
            debt: amount.to_string(),
        })),
        Err(e) => Err(IndexerError::IndexerCustom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct SumParams {
    pub address: String,
}

pub async fn get_delegator_sum(
    State(state): State<Arc<AppState>>,
    params: Query<SumParams>,
) -> Result<Json<DelegatorSumResponse>> {
    let mut pool = state.pool.acquire().await?;
    let address = params.0.address;
    let sql_query = format!(
        "SELECT (SELECT sum(amount) FROM evm_delegations WHERE delegator='{}') as sd, \
    (SELECT sum(amount) FROM evm_undelegations WHERE delegator='{}') as sund, \
    (SELECT sum(amount) FROM evm_coinbase_mint WHERE delegator='{}') as sc",
        address, address, address
    );
    let row = sqlx::query(&sql_query).fetch_one(&mut *pool).await?;
    let sum_delegate: BigDecimal = row.try_get("sd").unwrap_or_default();
    let sum_undelegate: BigDecimal = row.try_get("sund").unwrap_or_default();
    let sum_claim: BigDecimal = row.try_get("sc").unwrap_or_default();

    Ok(Json(DelegatorSumResponse {
        delegate: sum_delegate.to_string(),
        undelegate: sum_undelegate.to_string(),
        claim: sum_claim.to_string(),
    }))
}
