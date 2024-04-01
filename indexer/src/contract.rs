use crate::error::{IndexerError, Result};
use crate::types::{
    BoundResponse, DebtResponse, DelegatorSumResponse, RewardResponse, ValidatorDataResponse,
};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use ethers::types::H160;
use ethers::utils::hex;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::str::FromStr;
use std::sync::Arc;

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
        Err(e) => Err(IndexerError::Custom(e.to_string())),
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
    let staking = state.staking.clone();
    let validator = H160::from_str(&params.validator)?;
    let delegator = H160::from_str(&params.delegator)?;

    match staking.delegators(validator, delegator).call().await {
        Ok(info) => Ok(Json(BoundResponse {
            bound_amount: info.0.to_string(),
            unbound_amount: info.1.to_string(),
        })),
        Err(e) => return Err(IndexerError::Custom(e.to_string())),
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

    match reward.rewards(delegator).call().await {
        Ok(amount) => Ok(Json(RewardResponse {
            reward: amount.to_string(),
        })),
        Err(e) => return Err(IndexerError::Custom(e.to_string())),
    }
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
        Err(e) => Err(IndexerError::Custom(e.to_string())),
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

    let sql_delegate = r#"SELECT sum(amount) as s FROM evm_delegations WHERE delegator=$1"#;
    let sql_undelegate = r#"SELECT sum(amount) as s FROM evm_undelegations WHERE delegator=$1"#;
    let sql_claim = r#"SELECT sum(amount) as s FROM evm_coinbase_mint WHERE delegator=$1"#;

    let sum_delegate: BigDecimal = sqlx::query(sql_delegate)
        .bind(&params.0.address)
        .fetch_one(&mut *pool)
        .await?
        .try_get("s")
        .unwrap_or_default();

    let sum_undelegate: BigDecimal = sqlx::query(sql_undelegate)
        .bind(&params.0.address)
        .fetch_one(&mut *pool)
        .await?
        .try_get("s")
        .unwrap_or_default();

    let sum_claim: BigDecimal = sqlx::query(sql_claim)
        .bind(&params.0.address)
        .fetch_one(&mut *pool)
        .await?
        .try_get("s")
        .unwrap_or_default();

    Ok(Json(DelegatorSumResponse {
        delegate: sum_delegate.to_string(),
        undelegate: sum_undelegate.to_string(),
        claim: sum_claim.to_string(),
    }))
}
