mod api;
mod error;
mod types;

use crate::api::{
    get_claim_records, get_delegation_records, get_delegator_bound, get_delegator_debt,
    get_delegator_reward, get_delegators_of_validator, get_sum, get_undelegation_records,
    get_validator_detail, get_validator_status, get_validators, get_validators_of_delegator,
};
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use error::Result;
use ethers::addressbook::Address;
use ethers::contract::abigen;
use ethers::prelude::{Http, Provider};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolOptions;
use sqlx::{PgPool, Pool, Postgres};
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

abigen!(RewardContract, "../abi/Reward.json");
abigen!(StakingContract, "../abi/Staking.json");

#[derive(Serialize, Deserialize)]
struct Config {
    pub evm_rpc: String,
    pub staking: String,
    pub reward: String,
    pub listen: String,
    pub db_url: String,
}
impl Config {
    pub fn new(file_path: &str) -> Result<Self> {
        let mut f = File::open(file_path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let c: Config = toml::from_str(&s)?;
        Ok(c)
    }
}

struct AppState {
    //rds_conn: redis::Connection,
    pub pool: PgPool,
    pub staking: StakingContract<Provider<Http>>,
    pub reward: RewardContract<Provider<Http>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::new("./config.toml")?;
    info!("listen at: {}", config.listen);
    info!("EVM RPC: {}", config.evm_rpc);
    info!("staking contract: {}", config.staking);
    info!("reward contract: {}", config.reward);

    let provider = Provider::<Http>::try_from(config.evm_rpc)?;
    let staking_addr: Address = config.staking.parse()?;
    let staking = StakingContract::new(staking_addr, Arc::new(provider.clone()));
    let reward_addr: Address = config.reward.parse()?;
    let reward = RewardContract::new(reward_addr, Arc::new(provider));

    let db_url = env::var("DATABASE_URL").expect("missing env var `DATABASE_URL`");
    let pool: Pool<Postgres> = PoolOptions::new()
        .max_connections(10)
        .connect(db_url.as_str())
        .await
        .expect("can't connect to database");
    info!("Connecting db...ok");

    let app_state = Arc::new(AppState {
        pool,
        staking,
        reward,
    });
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/validator/list", get(get_validators))
        .route("/api/validator/detail", get(get_validator_detail))
        .route("/api/validator/status", get(get_validator_status))
        .route("/api/claims", get(get_claim_records))
        .route("/api/delegations", get(get_delegation_records))
        .route("/api/undelegations", get(get_undelegation_records))
        .route("/api/delegators", get(get_delegators_of_validator))
        .route("/api/validators", get(get_validators_of_delegator))
        .route("/api/bound", get(get_delegator_bound))
        .route("/api/reward", get(get_delegator_reward))
        .route("/api/debt", get(get_delegator_debt))
        .route("/api/sum", get(get_sum))
        .layer(cors)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(config.listen).await.unwrap();

    info!("Starting server...ok");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
