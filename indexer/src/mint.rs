use crate::error::Result;
use crate::types::{MintResponse, QueryResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct DelegatorMintRecordsParams {
    pub delegator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_delegator_mint_records(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorMintRecordsParams>,
) -> Result<Json<QueryResult<Vec<MintResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) = if let Some(delegator) = params.0.delegator {
        (
            format!(
                "SELECT count(*) FROM evm_coinbase_mint WHERE delegator='{}'",
                delegator
            ),
            format!(
                "SELECT tx_id,block_num,tm,validator,delegator,amount FROM evm_coinbase_mint \
            WHERE delegator='{}' ORDER BY tm DESC LIMIT {} OFFSET {}",
                delegator,
                page_size,
                (page - 1) * page_size
            ),
        )
    } else {
        (
            "SELECT count(*) FROM evm_coinbase_mint".to_string(),
            format!(
                "SELECT tx_id,block_num,tm,validator,delegator,amount FROM evm_coinbase_mint \
            ORDER BY tm DESC LIMIT {} OFFSET {}",
                page_size,
                (page - 1) * page_size
            ),
        )
    };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("count")?;

    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    let mut res: Vec<MintResponse> = vec![];
    for r in rows {
        let block_num: i64 = r.try_get("block_num")?;
        let tm: NaiveDateTime = r.try_get("tm")?;
        let tx: String = r.try_get("tx_id")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("amount")?;
        res.push(MintResponse {
            block_num,
            timestamp: tm.and_utc().timestamp(),
            tx,
            validator,
            delegator,
            amount: amount.to_string(),
        })
    }
    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: res,
    }))
}
