mod db;
mod error;
mod updater;

use crate::db::Storage;
use crate::error::Result;
use crate::updater::Updater;
use clap::Parser;
use ethers::contract::abigen;
use ethers::prelude::{Http, Provider};
use ethers::types::Address;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolOptions;
use sqlx::{Pool, Postgres};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_RPC_RETRIES: usize = 3;
const DEFAULT_INTERVAL: u64 = 13; // 13s

abigen!(RewardContract, "../abi/Reward.json");
abigen!(StakingContract, "../abi/Staking.json");

#[derive(Serialize, Deserialize)]
struct UpdaterConfig {
    pub evm_rpc: String,
    pub staking: String,
    pub reward: String,
    pub db_url: String,
}

impl UpdaterConfig {
    pub fn new(file_path: &str) -> Result<Self> {
        let mut f = File::open(file_path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let c: UpdaterConfig = toml::from_str(&s)?;
        Ok(c)
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Node RPC
    #[arg(long)]
    pub node: Option<String>,
    /// Block height to start scanning
    #[arg(long)]
    pub start: Option<u64>,
    /// Interval of scanning in seconds
    #[arg(long)]
    pub interval: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = UpdaterConfig::new("./config.toml")?;
    info!("EVM RPC: {}", config.evm_rpc);
    info!("Staking contract: {}", config.staking);
    info!("Reward contract: {}", config.reward);

    let pool: Pool<Postgres> = PoolOptions::new()
        .connect(&config.db_url)
        .await
        .expect("can't connect to database");
    info!("Connecting db...ok");

    let storage = Storage::new(pool);
    let args = Args::parse();
    let interval = if let Some(interval) = args.interval {
        Duration::from_secs(interval)
    } else {
        Duration::from_secs(DEFAULT_INTERVAL)
    };

    let provider = Provider::<Http>::try_from(config.evm_rpc)?;
    let staking_addr: Address = config.staking.parse()?;
    let staking = StakingContract::new(staking_addr, Arc::new(provider.clone()));
    let reward_addr: Address = config.reward.parse()?;
    let reward = RewardContract::new(reward_addr, Arc::new(provider.clone()));
    info!("Updating interval: {}s", interval.as_secs());
    let updater = Updater::new(DEFAULT_RPC_RETRIES, provider, staking, reward, storage);
    let _ = updater.run().await?;

    Ok(())
}
