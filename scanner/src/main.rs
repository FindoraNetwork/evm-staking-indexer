mod db;
mod error;
mod scanner;
mod tdrpc;
mod types;

extern crate num_cpus;
use crate::db::Storage;
use crate::scanner::Scanner;
use clap::Parser;
use error::Result;
use log::{error, info};
use reqwest::Url;
use sqlx::pool::PoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

const DEFAULT_INTERVAL: u64 = 12; // 12s
const DEFAULT_RPC_RETRIES: usize = 3;

#[derive(Parser, Debug)]
struct Args {
    /// Node RPC
    #[arg(long)]
    pub node: String,
    /// Pull single block
    #[arg(long)]
    pub single: bool,
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

    let db_url = env::var("DATABASE_URL").expect("Not find `DATABASE_URL`");
    let pool: Pool<Postgres> = PoolOptions::new()
        .max_connections(10)
        .connect(db_url.as_str())
        .await
        .expect("connect db failed");

    info!("connecting db...ok");
    let storage = Storage::new(pool);
    let args = Args::parse();
    let start = if let Some(start) = args.start {
        start
    } else {
        storage.get_tip().await.unwrap_or(4636000)
    };
    let interval = if let Some(interval) = args.interval {
        Duration::from_secs(interval)
    } else {
        Duration::from_secs(DEFAULT_INTERVAL)
    };
    info!("Node RPC: {}", args.node);
    info!("Scanning interval: {}s", interval.as_secs());
    info!("Starting from block: {}", start);

    let rpc_url: Url = args.node.parse().expect("parse node url");

    info!("Starting syncing...");
    let scanner = Scanner::new(DEFAULT_RPC_RETRIES, num_cpus::get(), rpc_url, storage)
        .expect("failed to new scanner");
    let _ = scanner.run(start, interval, args.single).await;

    Ok(())
}
