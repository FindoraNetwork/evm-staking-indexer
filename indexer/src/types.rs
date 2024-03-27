use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorVoteResponse {
    pub block_num: i64,
    pub should_vote: i32,
    pub voted: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorSumResponse {
    pub delegate: String,
    pub undelegate: String,
    pub claim: String,
}

#[derive(Serialize, Deserialize)]
pub struct BoundResponse {
    pub bound_amount: String,
    pub unbound_amount: String,
}

#[derive(Serialize, Deserialize)]
pub struct RewardResponse {
    pub reward: String,
}

#[derive(Serialize, Deserialize)]
pub struct DebtResponse {
    pub debt: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorLatest20Response {
    pub block_num: i64,
    pub total: String,
    pub delegator: String,
    pub amount: String,
    pub op: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorResponse {
    pub validator: String,
    pub staker: String,
    pub active: bool,
    pub jailed: bool,
    pub should_vote: i32,
    pub voted: i32,
    pub pubkey: String,
    pub pubkey_type: i32,
    pub rate: String,
    pub power: String,
    pub unbound_amount: String,
    pub punish_rate: String,
    pub begin_block: i64,
    pub unjail_time: i64,
    pub memo: Value,
}

#[derive(Serialize, Deserialize)]
pub struct ReceiptResponse {
    pub tx_id: String,
    pub block_id: String,
    pub block_num: i64,
    pub from: String,
    pub to: String,
    pub datetime: String,
    pub timestamp: i64,
    pub value: Value,
}

#[derive(Serialize, Deserialize)]
pub struct StakeResponse {
    pub tx_id: String,
    pub block_id: String,
    pub block_num: i64,
    pub datetime: String,
    pub timestamp: i64,
    pub validator: String,
    pub public_key: String,
    pub ty: i32,
    pub staker: String,
    pub amount: String,
    pub rate: String,
    pub memo: Value,
}

#[derive(Serialize, Deserialize)]
pub struct DelegateResponse {
    pub block_hash: String,
    pub validator: String,
    pub delegator: String,
    pub amount: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UndelegateResponse {
    pub block_hash: String,
    pub validator: String,
    pub delegator: String,
    pub amount: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorOfValidatorResponse {
    pub delegator: String,
    pub sum_amount: String,
}
