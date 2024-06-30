use crate::error::Result;
use crate::types::{
    DelegatorOfValidatorResponse, QueryResult, ValidatorLatest20Response, ValidatorResponse,
    ValidatorSumRewardResponse, ValidatorVoteResponse,
};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::ops::{Add, Sub};
use std::sync::Arc;

const SQL_QUERY1: &'static str = "ORDER BY power DESC LIMIT 1 OFFSET 0";

#[derive(Serialize, Deserialize)]
pub struct ValidatorSumRewardParams {
    pub validator: String,
}

pub async fn get_validator_sum_reward(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorSumRewardParams>,
) -> Result<Json<ValidatorSumRewardResponse>> {
    let mut pool = state.pool.acquire().await?;

    let sql_query = r#"SELECT sum(amount) FROM evm_coinbase_mint WHERE validator=$1"#;
    let row = sqlx::query(sql_query)
        .bind(&params.validator)
        .fetch_one(&mut *pool)
        .await?;
    let amount: Option<BigDecimal> = row.try_get("sum")?;

    Ok(Json(ValidatorSumRewardResponse {
        reward: amount.unwrap_or_default().to_string(),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct GetVoteParams {
    pub validator: String,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

const BLOCKS_PER_DAY: i64 = 24 * 60 * 60 / 15; // 5760

pub async fn get_validator_votes(
    State(state): State<Arc<AppState>>,
    params: Query<GetVoteParams>,
) -> Result<Json<QueryResult<Vec<ValidatorVoteResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let sql_total = r#"SELECT count(block_num) FROM evm_validators WHERE validator = $1
        AND block_num >= (SELECT max(block_num) FROM evm_validators) - $2"#;
    let row = sqlx::query(sql_total)
        .bind(&params.0.validator)
        .bind(BLOCKS_PER_DAY)
        .fetch_one(&mut *pool)
        .await?;
    let total: i64 = row.try_get("count")?;

    let sql_query = r#"SELECT block_num,should_vote,voted FROM evm_validators WHERE
        validator = $1 AND block_num >= (SELECT max(block_num) FROM evm_validators) - $2
        ORDER BY block_num DESC LIMIT $3 OFFSET $4"#;
    let rows = sqlx::query(sql_query)
        .bind(&params.0.validator)
        .bind(BLOCKS_PER_DAY)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *pool)
        .await?;
    let mut votes: Vec<ValidatorVoteResponse> = vec![];
    for r in rows {
        let block_num: i64 = r.try_get("block_num")?;
        let should_vote: i32 = r.try_get("should_vote")?;
        let voted: i32 = r.try_get("voted")?;
        votes.push(ValidatorVoteResponse {
            block_num,
            should_vote,
            voted,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: votes,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct GetValidatorsParams {
    pub validator: Option<String>,
    pub online: Option<bool>,
    pub offline: Option<bool>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_validators(
    State(state): State<Arc<AppState>>,
    params: Query<GetValidatorsParams>,
) -> Result<Json<QueryResult<Vec<ValidatorResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut sql_total = "SELECT count(block_num) \
        FROM evm_validators ev \
        LEFT JOIN evm_stakes es \
        ON ev.validator=es.validator \
        WHERE ev.block_num=(SELECT max(block_num) FROM evm_validators) "
        .to_string();

    let mut sql_query = "SELECT ev.validator,ev.pubkey,ev.pubkey_type,ev.rate,ev.staker,ev.power,\
        ev.unbound,ev.punish_rate,ev.begin_block,ev.active,ev.jailed,ev.unjail_time,ev.should_vote,ev.voted,es.memo \
        FROM evm_validators ev \
        LEFT JOIN evm_stakes es \
        ON ev.validator=es.validator \
        WHERE ev.block_num=(SELECT max(block_num) FROM evm_validators) ".to_string();

    let mut query_params: Vec<String> = vec![];
    if let Some(ref validator) = params.0.validator {
        query_params.push(format!("ev.validator='{}' ", validator));
    }
    if let Some(online) = params.0.online {
        query_params.push(format!("ev.active={} ", online))
    }
    if let Some(offline) = params.0.offline {
        query_params.push(format!("ev.jailed={} ", offline))
    }
    if !query_params.is_empty() {
        sql_total = sql_total
            .add("AND ")
            .add(query_params.join("AND ").as_str());
        sql_query = sql_query
            .add("AND ")
            .add(query_params.join("AND ").as_str());
    }
    if params.0.validator.is_some() {
        sql_query = sql_query.add(SQL_QUERY1);
    } else {
        sql_query = sql_query.add(
            format!(
                "ORDER BY power DESC LIMIT {} OFFSET {}",
                page_size,
                (page - 1) * page_size
            )
            .as_str(),
        );
    }

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("count")?;

    let mut validators: Vec<ValidatorResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    for r in rows {
        let validator: String = r.try_get("validator")?;
        let staker: String = r.try_get("staker")?;
        let active: bool = r.try_get("active")?;
        let jailed: bool = r.try_get("jailed")?;
        let should_vote: i32 = r.try_get("should_vote")?;
        let voted: i32 = r.try_get("voted")?;
        let pubkey: String = r.try_get("pubkey")?;
        let pubkey_type: i32 = r.try_get("pubkey_type")?;
        let rate: BigDecimal = r.try_get("rate")?;
        let power: BigDecimal = r.try_get("power")?;
        let unbound_amount: BigDecimal = r.try_get("unbound")?;
        let punish_rate: BigDecimal = r.try_get("punish_rate")?;
        let begin_block: i64 = r.try_get("begin_block")?;
        let unjail_time: NaiveDateTime = r.try_get("unjail_time")?;
        let memo: Value = r.try_get("memo")?;
        validators.push(ValidatorResponse {
            validator,
            staker,
            active,
            jailed,
            should_vote,
            voted,
            pubkey,
            pubkey_type,
            rate: rate.to_string(),
            power: power.to_string(),
            unbound_amount: unbound_amount.to_string(),
            punish_rate: punish_rate.to_string(),
            begin_block,
            unjail_time: unjail_time.and_utc().timestamp(),
            memo,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: validators,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct GetLatest20Params {
    pub validator: String,
}

pub async fn get_latest20(
    State(state): State<Arc<AppState>>,
    params: Query<GetLatest20Params>,
) -> Result<Json<Vec<ValidatorLatest20Response>>> {
    let mut pool = state.pool.acquire().await?;
    let mut latest: Vec<ValidatorLatest20Response> = vec![];

    let sql_query = r#"SELECT block_num,delegator,amount,op from evm_audit WHERE validator=$1 ORDER BY block_num DESC LIMIT 20"#;
    let rows = sqlx::query(sql_query)
        .bind(&params.0.validator)
        .fetch_all(&mut *pool)
        .await?;
    if rows.len() == 0 {
        return Ok(Json(latest));
    };

    let max_height: i64 = rows[0].try_get("block_num")?;
    let sql_sum = r#"SELECT sum(amount) FROM evm_audit WHERE validator=$1 and block_num<=$2"#;
    let row = sqlx::query(sql_sum)
        .bind(&params.0.validator)
        .bind(max_height)
        .fetch_one(&mut *pool)
        .await?;
    let mut sum: BigDecimal = row.try_get("sum")?;
    for i in 0..rows.len() {
        let block_num: i64 = rows[i].try_get("block_num")?;
        let amount: BigDecimal = rows[i].try_get("amount")?;
        let delegator: String = rows[i].try_get("delegator")?;
        let op: i32 = rows[i].try_get("op")?;

        latest.push(ValidatorLatest20Response {
            block_num,
            total: sum.to_string(),
            delegator,
            amount: amount.to_string(),
            op,
        });

        sum = sum.clone().sub(amount.clone());
    }

    Ok(Json(latest))
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
) -> Result<Json<QueryResult<Vec<DelegatorOfValidatorResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let sql_total = r#"SELECT count(distinct delegator) FROM evm_audit WHERE validator=$1"#;
    let sql_query = r#"SELECT DISTINCT delegator,sum(amount),dense_rank() over (order by sum(amount) desc) rank
        FROM evm_audit WHERE validator=$1 GROUP BY delegator ORDER BY rank LIMIT $2 OFFSET $3"#;
    let row = sqlx::query(sql_total)
        .bind(&params.0.validator)
        .fetch_one(&mut *pool)
        .await?;
    let total: i64 = row.try_get("count")?;
    let rows = sqlx::query(sql_query)
        .bind(&params.0.validator)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *pool)
        .await?;

    let mut res: Vec<DelegatorOfValidatorResponse> = vec![];
    for r in rows {
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("sum")?;
        let rank: i64 = r.try_get("rank")?;

        res.push(DelegatorOfValidatorResponse {
            delegator,
            amount: amount.to_string(),
            rank,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: res,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorOfDelegatorParams {
    pub delegator: String,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_validators_of_delegator(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorOfDelegatorParams>,
) -> Result<Json<QueryResult<Vec<String>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let sql_total = r#"SELECT count(distinct validator) FROM evm_audit WHERE delegator=$1"#;
    let row = sqlx::query(sql_total)
        .bind(&params.0.delegator)
        .fetch_one(&mut *pool)
        .await?;
    let total: i64 = row.try_get("count")?;

    let sql_query =
        r#"SELECT distinct validator FROM evm_audit WHERE delegator=$1 LIMIT $2 OFFSET $3"#;
    let rows = sqlx::query(sql_query)
        .bind(&params.0.delegator)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *pool)
        .await?;
    let mut validators: Vec<String> = vec![];
    for r in rows {
        let validator: String = r.try_get("validator")?;
        validators.push(validator);
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: validators,
    }))
}
