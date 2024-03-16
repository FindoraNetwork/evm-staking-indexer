use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Parts {
    pub total: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BlockId {
    pub hash: String,
    pub parts: Parts,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Version {
    pub block: String,
    pub app: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BlockHeader {
    pub version: Version,
    pub chain_id: String,
    pub height: String,
    pub time: String,
    pub last_block_id: BlockId,
    pub last_commit_hash: String,
    pub data_hash: String,
    pub validators_hash: String,
    pub next_validators_hash: String,
    pub consensus_hash: String,
    pub app_hash: String,
    pub last_results_hash: String,
    pub evidence_hash: String,
    pub proposer_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub txs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LastCommit {
    pub height: String,
    pub round: String,
    pub block_id: BlockId,
    pub signatures: Option<Vec<Signature>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Signature {
    pub validator_address: Option<String>,
    pub timestamp: Option<String>,
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Block {
    pub header: BlockHeader,
    pub data: Data,
    pub last_commit: LastCommit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: i64,
    pub result: T,
}

#[derive(Serialize, Deserialize)]
pub struct RpcBlock {
    pub block_id: BlockId,
    pub block: Block,
}

#[derive(Serialize, Deserialize)]
pub struct TxResult {
    pub code: i64,
    pub data: Option<String>,
    pub log: String,
    pub info: String,
    #[serde(rename = "gasWanted")]
    pub gas_wanted: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub events: Vec<Value>,
    #[serde(rename = "codespace")]
    pub code_space: String,
}

#[derive(Serialize, Deserialize)]
pub enum ContractOp {
    Create(CreateData),
    Call(CallData),
}

#[derive(Serialize, Deserialize)]
pub struct CallData {
    #[serde(rename = "Call")]
    pub call: EvmResult,
}

#[derive(Serialize, Deserialize)]
pub struct CreateData {
    #[serde(rename = "Create")]
    pub create: EvmResult,
}

#[derive(Serialize, Deserialize)]
pub struct EvmResult {
    pub exit_reason: Value,
    pub value: Value,
    pub used_gas: String,
    pub logs: Vec<TxCallLog>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateContract {}

#[derive(Serialize, Deserialize)]
pub struct TxCallLog {
    pub address: String,
    pub topics: Vec<String>,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct RpcTransaction {
    pub hash: String,
    pub height: String,
    pub index: i64,
    pub tx: String,
    pub tx_result: TxResult,
}
