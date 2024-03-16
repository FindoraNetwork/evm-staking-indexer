use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ValidatorResponse {
    pub address: String,
    pub power: String,
    pub public_key: String,
    pub public_key_type: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorStatusResponse {
    pub heap_index_off1: String,
    pub is_active: bool,
    pub jailed: bool,
    pub unjail_datetime: u64,
    pub should_vote: i32,
    pub voted: i32,
}
#[derive(Serialize, Deserialize)]
pub struct ValidatorDataResponse {
    pub public_key: String,
    pub public_key_type: i32,
    pub rate: String,
    pub staker: String,
    pub power: String,
    pub total_unbound_amount: String,
    pub punish_rate: String,
    pub begin_block: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct DelegationRecord {
    pub tx: String,
    pub block_num: i64,
    pub validator: String,
    pub delegator: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize)]
pub struct UndelegationRecord {
    pub tx: String,
    pub block_num: i64,
    pub index: i64,
    pub validator: String,
    pub delegator: String,
    pub unlock_time: i64,
    pub amount: String,
    pub op_type: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorInfo {
    pub bound_amount: String,
    pub unbound_amount: String,
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorReward {
    pub reward: String,
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorDebt {
    pub debt: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorAmount {
    pub address: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorSum {
    pub sum: String,
    pub validators: Vec<ValidatorAmount>,
}

#[derive(Serialize, Deserialize)]
pub struct DelegatorSum {
    pub sum_delegate: String,
    pub sum_undelegate: String,
    pub sum_claim: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimRecord {
    pub tx: String,
    pub block_num: i64,
    pub validator: String,
    pub delegator: String,
    pub amount: String,
}
