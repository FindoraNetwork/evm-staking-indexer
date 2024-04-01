use crate::error::Result;
use crate::types::{DelegateResponse, QueryResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct DelegatorDelegateRecordsParams {
    pub delegator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_delegator_delegate_records(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorDelegateRecordsParams>,
) -> Result<Json<QueryResult<Vec<DelegateResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) = if let Some(delegator) = params.0.delegator {
        (
            format!("SELECT count(*) FROM evm_delegations WHERE delegator='{}'", delegator)
            ,
            format!("SELECT block_id,validator,delegator,tm,amount FROM evm_delegations WHERE delegator='{}' ORDER BY tm DESC LIMIT {} OFFSET {}", delegator, page_size, (page-1)*page_size)
        )
    } else {
        (
            "SELECT count(*) FROM evm_delegations".to_string()
            ,
            format!("SELECT block_id,validator,delegator,tm,amount FROM evm_delegations ORDER BY tm DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size))
    };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("count")?;

    let mut delegates: Vec<DelegateResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    for r in rows {
        let block_hash: String = r.try_get("block_id")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("amount")?;
        let tm: NaiveDateTime = r.try_get("tm")?;

        delegates.push(DelegateResponse {
            block_hash,
            validator,
            delegator,
            amount: amount.to_string(),
            timestamp: tm.and_utc().timestamp(),
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: delegates,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorDelegateRecordsParams {
    pub validator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_validator_delegate_records(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorDelegateRecordsParams>,
) -> Result<Json<QueryResult<Vec<DelegateResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) = if let Some(validator) = params.0.validator {
        (
            format!("SELECT count(*) FROM evm_delegations WHERE validator='{}'", validator)
            ,
            format!("SELECT block_id,validator,delegator,tm,amount FROM evm_delegations WHERE validator='{}' ORDER BY tm DESC LIMIT {} OFFSET {}", validator, page_size, (page-1)*page_size)
        )
    } else {
        (
            "SELECT count(*) FROM evm_delegations".to_string()
            ,
            format!("SELECT block_id,validator,delegator,tm,amount FROM evm_delegations ORDER BY tm DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size))
    };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("count")?;

    let mut delegates: Vec<DelegateResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    for r in rows {
        let block_hash: String = r.try_get("block_id")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("amount")?;
        let tm: NaiveDateTime = r.try_get("tm")?;

        delegates.push(DelegateResponse {
            block_hash,
            validator,
            delegator,
            amount: amount.to_string(),
            timestamp: tm.and_utc().timestamp(),
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: delegates,
    }))
}
