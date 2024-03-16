use crate::error::{IndexerError, Result};
use crate::types::{
    ClaimRecord, DelegationRecord, DelegatorDebt, DelegatorInfo, DelegatorReward, DelegatorSum,
    QueryResult, UndelegationRecord, ValidatorDataResponse, ValidatorResponse,
    ValidatorStatusResponse,
};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use ethers::types::H160;
use ethers::utils::hex;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct ClaimRecordsParams {
    pub delegator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_claim_records(
    State(state): State<Arc<AppState>>,
    params: Query<ClaimRecordsParams>,
) -> Result<Json<QueryResult<Vec<ClaimRecord>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) = if let Some(addr) = params.0.delegator {
        (
            format!("select count(*) as cnt from evm_e_coinbase_mint where delegator='{}'", addr),
            format!("select tx,block_num,validator,delegator,amount from evm_e_coinbase_mint where delegator='{}' order by block_num desc limit {} offset {}", addr, page_size, (page-1)*page_size)
        )
    } else {
        ("select count(*) as cnt from evm_e_coinbase_mint".to_string(),
        format!("select tx,block_num,validator,delegator,amount from evm_e_coinbase_mint order by block_num desc limit {} offset {}",page_size, (page-1)*page_size))
    };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("cnt")?;

    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    let mut claims: Vec<ClaimRecord> = vec![];
    for r in rows {
        let tx: String = r.try_get("tx")?;
        let block_num: i64 = r.try_get("block_num")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("amount")?;
        claims.push(ClaimRecord {
            tx,
            block_num,
            validator,
            delegator,
            amount: amount.to_string(),
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: claims,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct DelegationRecordsParams {
    pub delegator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_delegation_records(
    State(state): State<Arc<AppState>>,
    params: Query<DelegationRecordsParams>,
) -> Result<Json<QueryResult<Vec<DelegationRecord>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) =
        if let Some(addr) = params.0.delegator {
            (
                format!(
                    "select count(*) as cnt from evm_e_delegation where delegator='{}'",
                    addr
                ),
                format!(
                    "select tx,block_num,validator,delegator,amount from evm_e_delegation where \
                delegator='{}' order by block_num desc limit {} offset {}",
                    addr,
                    page_size,
                    (page - 1) * page_size
                ),
            )
        } else {
            (
            "select count(*) as cnt from evm_e_delegation".to_string(),
            format!("select tx,block_num,validator,delegator,amount from evm_e_delegation order by \
                block_num desc limit {} offset {}", page_size, (page-1)*page_size)
        )
        };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("cnt")?;

    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;

    let mut delegations: Vec<DelegationRecord> = vec![];
    for r in rows {
        let tx: String = r.try_get("tx")?;
        let block_num: i64 = r.try_get("block_num")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("amount")?;
        delegations.push(DelegationRecord {
            tx,
            block_num,
            validator,
            delegator,
            amount: amount.to_string(),
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: delegations,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct UndelegationRecordsParams {
    pub delegator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_undelegation_records(
    State(state): State<Arc<AppState>>,
    params: Query<UndelegationRecordsParams>,
) -> Result<Json<QueryResult<Vec<UndelegationRecord>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) = if let Some(addr) = params.0.delegator {
        (
            format!(
                "select count(*) as cnt from evm_e_undelegation where delegator='{}'",
                addr
            ),
            format!(
                "select tx,block_num,idx,validator,delegator,unlock_time,amount,op_type from \
                evm_e_undelegation where delegator='{}' order by block_num desc limit {} offset {}",
                addr,
                page_size,
                (page - 1) * page_size
            ),
        )
    } else {
        (
            "select count(*) as cnt from evm_e_undelegation".to_string(),
            format!(
                "select tx,block_num,idx,validator,delegator,unlock_time,amount,op_type from \
                evm_e_undelegation order by block_num desc limit {} offset {}",
                page_size,
                (page - 1) * page_size
            ),
        )
    };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("cnt")?;

    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    let mut undelegations: Vec<UndelegationRecord> = vec![];
    for r in rows {
        let tx: String = r.try_get("tx")?;
        let block_num: i64 = r.try_get("block_num")?;
        let index: i64 = r.try_get("idx")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let unlock_time: i64 = r.try_get("unlock_time")?;
        let amount: BigDecimal = r.try_get("amount")?;
        let op_type: i32 = r.try_get("op_type")?;
        undelegations.push(UndelegationRecord {
            tx,
            block_num,
            index,
            validator,
            delegator,
            unlock_time,
            amount: amount.to_string(),
            op_type,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: undelegations,
    }))
}

pub async fn get_validators(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ValidatorResponse>>> {
    let staking = state.staking.clone();
    match staking.get_validators_list().call().await {
        Ok(vinfos) => {
            let mut validators: Vec<ValidatorResponse> = vec![];
            for info in vinfos {
                validators.push(ValidatorResponse {
                    address: hex::encode(info.addr.as_bytes()),
                    power: info.power.to_string(),
                    public_key: info.public_key.to_string(),
                    public_key_type: info.ty as i32,
                });
            }
            Ok(Json(validators))
        }
        Err(e) => Err(IndexerError::Custom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorStatusParams {
    pub validator: String,
}

#[debug_handler]
pub async fn get_validator_status(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorStatusParams>,
) -> Result<Json<ValidatorStatusResponse>> {
    let staking = state.staking.clone();
    let addr = H160::from_str(&params.0.validator)?;

    match staking.validator_status(addr).call().await {
        Ok(vs) => {
            let res = ValidatorStatusResponse {
                heap_index_off1: vs.0.to_string(),
                is_active: vs.1,
                jailed: vs.2,
                unjail_datetime: vs.3,
                should_vote: vs.4 as i32,
                voted: vs.5 as i32,
            };
            Ok(Json(res))
        }
        Err(e) => Err(IndexerError::Custom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorDetailParams {
    pub validator: String,
}

pub async fn get_validator_detail(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorDetailParams>,
) -> Result<Json<ValidatorDataResponse>> {
    let staking = state.staking.clone();
    let addr = H160::from_str(&params.0.validator)?;
    match staking.validators(addr).call().await {
        Ok(vd) => {
            let res = ValidatorDataResponse {
                public_key: vd.0.to_string(),
                public_key_type: vd.1 as i32,
                rate: vd.2.to_string(),
                staker: hex::encode_prefixed(vd.3.as_bytes()),
                power: vd.4.to_string(),
                total_unbound_amount: vd.5.to_string(),
                punish_rate: vd.6.to_string(),
                begin_block: vd.7.to_string(),
            };
            Ok(Json(res))
        }
        Err(e) => Err(IndexerError::Custom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorsOfValidatorParams {
    pub validator: String,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_delegators_of_validator(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorsOfValidatorParams>,
) -> Result<Json<QueryResult<Vec<String>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let sql_total =
        r#"select count(distinct delegator) as cnt from evm_e_delegation where validator=$1"#;
    let row = sqlx::query(sql_total)
        .bind(&params.0.validator)
        .fetch_one(&mut *pool)
        .await?;
    let total: i64 = row.try_get("cnt")?;

    let sql_query =
        r#"select distinct delegator from evm_e_delegation where validator=$1 limit $2 offset $3"#;
    let rows = sqlx::query(sql_query)
        .bind(&params.0.validator)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *pool)
        .await?;

    let mut delegators: Vec<String> = vec![];
    for r in rows {
        let d: String = r.try_get("delegator")?;
        delegators.push(d)
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: delegators,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorsOfDelegatorParams {
    pub delegator: String,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_validators_of_delegator(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorsOfDelegatorParams>,
) -> Result<Json<QueryResult<Vec<String>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let sql_total =
        r#"select count(distinct validator) as cnt from evm_e_delegation where delegator=$1"#;
    let row = sqlx::query(sql_total)
        .bind(&params.0.delegator)
        .fetch_one(&mut *pool)
        .await?;
    let total: i64 = row.try_get("cnt")?;

    let sql_query =
        r#"select distinct validator from evm_e_delegation where delegator=$1 limit $2 offset $3"#;
    let rows = sqlx::query(sql_query)
        .bind(&params.0.delegator)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *pool)
        .await?;

    let mut validators: Vec<String> = vec![];
    for r in rows {
        let d: String = r.try_get("validator")?;
        validators.push(d);
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: validators,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorInfoParams {
    pub validator: String,
    pub delegator: String,
}

pub async fn get_delegator_bound(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorInfoParams>,
) -> Result<Json<DelegatorInfo>> {
    let staking = state.staking.clone();
    let validator = H160::from_str(&params.validator)?;
    let delegator = H160::from_str(&params.delegator)?;

    match staking.delegators(validator, delegator).call().await {
        Ok(info) => Ok(Json(DelegatorInfo {
            bound_amount: info.0.to_string(),
            unbound_amount: info.1.to_string(),
        })),
        Err(e) => return Err(IndexerError::Custom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorRewardParams {
    pub delegator: String,
}

pub async fn get_delegator_reward(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorRewardParams>,
) -> Result<Json<DelegatorReward>> {
    let reward = state.reward.clone();
    let delegator = H160::from_str(&params.0.delegator)?;

    match reward.rewards(delegator).call().await {
        Ok(amount) => Ok(Json(DelegatorReward {
            reward: amount.to_string(),
        })),
        Err(e) => return Err(IndexerError::Custom(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorRewardDebtParams {
    pub validator: String,
    pub delegator: String,
}

pub async fn get_delegator_debt(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorRewardDebtParams>,
) -> Result<Json<DelegatorDebt>> {
    let reward = state.reward.clone();
    let validator = H160::from_str(&params.validator)?;
    let delegator = H160::from_str(&params.delegator)?;
    match reward.reward_debt(validator, delegator).call().await {
        Ok(amount) => Ok(Json(DelegatorDebt {
            debt: amount.to_string(),
        })),
        Err(e) => Err(IndexerError::Custom(e.to_string())),
    }
}
#[derive(Serialize, Deserialize)]
pub struct SumParams {
    pub delegator: String,
}
pub async fn get_sum(
    State(state): State<Arc<AppState>>,
    params: Query<SumParams>,
) -> Result<Json<DelegatorSum>> {
    let mut pool = state.pool.acquire().await?;

    let sql_delegate = r#"select sum(amount) as s from evm_e_delegation where delegator=$1"#;
    let sql_undelegate = r#"select sum(amount) as s from evm_e_undelegation where delegator=$1"#;
    let sql_claim = r#"select sum(amount) as s from evm_e_coinbase_mint where delegator=$1"#;

    let sum_delegate: BigDecimal = sqlx::query(sql_delegate)
        .bind(&params.0.delegator)
        .fetch_one(&mut *pool)
        .await?
        .try_get("s")?;
    let sum_undelegate: BigDecimal = sqlx::query(sql_undelegate)
        .bind(&params.0.delegator)
        .fetch_one(&mut *pool)
        .await?
        .try_get("s")?;
    let sum_claim: BigDecimal = sqlx::query(sql_claim)
        .bind(&params.0.delegator)
        .fetch_one(&mut *pool)
        .await?
        .try_get("s")?;

    Ok(Json(DelegatorSum {
        sum_delegate: sum_delegate.to_string(),
        sum_undelegate: sum_undelegate.to_string(),
        sum_claim: sum_claim.to_string(),
    }))
}
