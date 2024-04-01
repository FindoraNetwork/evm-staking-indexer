mod delegate;
mod error;
mod receipt;
mod stake;

mod contract;
mod types;
mod undelegate;
mod validators;

use crate::contract::{
    get_delegator_bound, get_delegator_debt, get_delegator_reward, get_delegator_sum,
    get_validator_data,
};
use crate::delegate::get_delegate_records;
use crate::receipt::get_receipts;
use crate::stake::get_stake_records;
use crate::undelegate::get_undelegate_records;
use crate::validators::{
    get_delegators_of_validator, get_latest20, get_validator_votes, get_validators,
    get_validators_of_delegator,
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
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

abigen!(RewardContract, "../abi/Reward.json");
abigen!(StakingContract, "../abi/Staking.json");

#[derive(Serialize, Deserialize)]
struct IndexerConfig {
    pub evm_rpc: String,
    pub staking: String,
    pub reward: String,
    pub listen: String,
    pub db_url: String,
}
impl IndexerConfig {
    pub fn new(file_path: &str) -> Result<Self> {
        let mut f = File::open(file_path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let c: IndexerConfig = toml::from_str(&s)?;
        Ok(c)
    }
}

struct AppState {
    pub pool: PgPool,
    pub staking: StakingContract<Provider<Http>>,
    pub reward: RewardContract<Provider<Http>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = IndexerConfig::new("./config.toml")?;
    info!("Listening at: {}", config.listen);
    info!("EVM RPC: {}", config.evm_rpc);
    info!("Staking contract: {}", config.staking);
    info!("Reward contract: {}", config.reward);

    let provider = Provider::<Http>::try_from(config.evm_rpc)?;
    let staking_addr: Address = config.staking.parse()?;
    let staking = StakingContract::new(staking_addr, Arc::new(provider.clone()));
    let reward_addr: Address = config.reward.parse()?;
    let reward = RewardContract::new(reward_addr, Arc::new(provider));

    let pool: Pool<Postgres> = PoolOptions::new()
        .connect(&config.db_url)
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
        .route("/api/validators", get(get_validators))
        .route(
            "/api/validator/delegators",
            get(get_delegators_of_validator),
        )
        .route(
            "/api/delegator/validators",
            get(get_validators_of_delegator),
        )
        .route("/api/diff/latest", get(get_latest20))
        .route("/api/records/delegate", get(get_delegate_records))
        .route("/api/records/undelegate", get(get_undelegate_records))
        .route("/api/diff/vote", get(get_validator_votes))
        .route("/api/records/stake", get(get_stake_records))
        .route("/api/receipts", get(get_receipts))
        .route("/api/bound", get(get_delegator_bound))
        .route("/api/reward", get(get_delegator_reward))
        .route("/api/debt", get(get_delegator_debt))
        .route("/api/sum", get(get_delegator_sum))
        .route("/api/vdata", get(get_validator_data))
        .layer(cors)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(config.listen).await.unwrap();

    info!("Starting server...ok");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
