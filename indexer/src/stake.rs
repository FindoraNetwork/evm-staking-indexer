use crate::error::Result;
use crate::types::{QueryResult, StakeResponse};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::ops::Add;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct GetStakesParams {
    pub txid: Option<String>,
    pub blockid: Option<String>,
    pub blocknum: Option<i64>,
    pub validator: Option<String>,
    pub staker: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_stake_records(
    State(state): State<Arc<AppState>>,
    params: Query<GetStakesParams>,
) -> Result<Json<QueryResult<Vec<StakeResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut sql_total = "SELECT count(block_num) FROM evm_stakes ".to_string();
    let mut sql_query =
        "SELECT tx_id,block_id,block_num,tm,validator,pubkey,ty,staker,amount,memo,rate FROM evm_stakes ".to_string();

    let mut query_params: Vec<String> = vec![];
    if let Some(tx_id) = params.0.txid {
        query_params.push(format!("tx_id='{}' ", tx_id))
    }
    if let Some(block_id) = params.0.blockid {
        query_params.push(format!("block_id='{}' ", block_id))
    }
    if let Some(block_num) = params.0.blocknum {
        query_params.push(format!("block_mum={} ", block_num))
    }
    if let Some(validator) = params.0.validator {
        query_params.push(format!("validator='{}' ", validator))
    }
    if let Some(staker) = params.0.staker {
        query_params.push(format!("staker='{}' ", staker))
    }

    if !query_params.is_empty() {
        sql_total = sql_total
            .add("WHERE ")
            .add(query_params.join("AND ").as_str());
        sql_query = sql_query
            .add("WHERE ")
            .add(query_params.join("AND ").as_str());
    }

    sql_query = sql_query.add(
        format!(
            "ORDER BY tm DESC LIMIT {} OFFSET {}",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("count")?;

    let mut stakes: Vec<StakeResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    for r in rows {
        let tx_id: String = r.try_get("tx_id")?;
        let block_id: String = r.try_get("block_id")?;
        let block_num: i64 = r.try_get("block_num")?;
        let tm: NaiveDateTime = r.try_get("tm")?;
        let validator: String = r.try_get("validator")?;
        let public_key: String = r.try_get("pubkey")?;
        let ty: i32 = r.try_get("ty")?;
        let staker: String = r.try_get("staker")?;
        let amount: BigDecimal = r.try_get("amount")?;
        let rate: BigDecimal = r.try_get("rate")?;
        let memo: Value = r.try_get("memo")?;

        stakes.push(StakeResponse {
            tx_id,
            block_id,
            block_num,
            datetime: tm.to_string(),
            timestamp: tm.and_utc().timestamp(),
            validator,
            public_key,
            ty,
            staker,
            amount: amount.to_string(),
            rate: rate.to_string(),
            memo,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: stakes,
    }))
}
