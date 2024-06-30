use crate::error::Result;
use crate::types::{QueryResult, ReceiptResponse};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use std::ops::Add;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct GetReceiptsParams {
    pub txid: Option<String>,
    pub blockid: Option<String>,
    pub blocknum: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_receipts(
    State(state): State<Arc<AppState>>,
    params: Query<GetReceiptsParams>,
) -> Result<Json<QueryResult<Vec<ReceiptResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut sql_total = "SELECT count(height) FROM evm_receipts ".to_string();
    let mut sql_query =
        "SELECT tx_id,block_id,block_num,from_addr,to_addr,tm,value FROM evm_receipts ".to_string();

    let mut query_params: Vec<String> = vec![];
    if let Some(tx_id) = params.0.txid {
        query_params.push(format!("tx_id='{}' ", tx_id))
    }
    if let Some(block_id) = params.0.blockid {
        query_params.push(format!("block_id='{}' ", block_id))
    }
    if let Some(block_num) = params.0.blocknum {
        query_params.push(format!("block_num={} ", block_num))
    }
    if let Some(from) = params.0.from {
        query_params.push(format!("from_addr='{}' ", from))
    }
    if let Some(to) = params.0.to {
        query_params.push(format!("to='{}' ", to))
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

    let mut receipts: Vec<ReceiptResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    for r in rows {
        let tx_id: String = r.try_get("tx_id")?;
        let block_id: String = r.try_get("block_id")?;
        let block_num: i64 = r.try_get("block_num")?;
        let from: String = r.try_get("from_addr")?;
        let to: String = r.try_get("to_addr")?;
        let tm: NaiveDateTime = r.try_get("tm")?;
        let value: Value = r.try_get("value")?;

        receipts.push(ReceiptResponse {
            tx_id,
            block_id,
            block_num,
            from,
            to,
            datetime: tm.to_string(),
            timestamp: tm.and_utc().timestamp(),
            value,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: receipts,
    }))
}
