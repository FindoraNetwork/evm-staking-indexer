use crate::error;
use error::Result;
use serde_json::Value;
use sqlx::types::BigDecimal;
use sqlx::{PgPool, Row};

#[derive(Debug)]
pub struct Storage {
    pool: PgPool,
}

impl Storage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_tip(&self) -> Result<u64> {
        let row = sqlx::query("select height from evm_last_height")
            .fetch_one(&self.pool)
            .await?;
        let height: i64 = row.try_get("height")?;

        Ok(height as u64)
    }

    pub async fn upsert_tip(&self, height: i64) -> Result<()> {
        sqlx::query(
            "insert into evm_last_height values($1,$2) on conflict(tip) do update set height=$2",
        )
        .bind("tip")
        .bind(height)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn upsert_stake(
        &self,
        tx: &str,
        block_num: i64,
        validator: &str,
        public_key: &str,
        ty: i32,
        staker: &str,
        amount: BigDecimal,
        memo: &str,
        rate: BigDecimal,
    ) -> Result<()> {
        sqlx::query("insert into evm_e_stake values($1,$2,$3,$4,$5,$6,$7,$8,$9) on \
            conflict(tx,validator,staker) do update set block_num=$2,public_key=$4,ty=$5,amount=$7,memo=$8,rate=$9")
        .bind(tx)
        .bind(block_num)
        .bind(validator)
        .bind(public_key)
        .bind(ty)
        .bind(staker)
        .bind(amount)
        .bind(memo)
        .bind(rate)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_delegation(
        &self,
        tx: &str,
        block_num: i64,
        validator: &str,
        delegator: &str,
        amount: BigDecimal,
    ) -> Result<()> {
        sqlx::query(
            "insert into evm_e_delegation values($1,$2,$3,$4,$5) on \
                conflict(tx,validator,delegator) do update set block_num=$2,amount=$5",
        )
        .bind(tx)
        .bind(block_num)
        .bind(validator)
        .bind(delegator)
        .bind(amount)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_undelegation(
        &self,
        tx: &str,
        block_num: i64,
        idx: i32,
        validator: &str,
        delegator: &str,
        unlock_time: i64,
        amount: BigDecimal,
        op_type: i32,
    ) -> Result<()> {
        sqlx::query("insert into evm_e_undelegation values($1,$2,$3,$4,$5,$6,$7,$8) on \
            conflict(tx,validator,delegator) do update set block_num=$2,idx=$3,unlock_time=$6,amount=$7,op_type=$8")
        .bind(tx)
        .bind(block_num)
        .bind(idx)
        .bind(validator)
        .bind(delegator)
        .bind(unlock_time)
        .bind(amount)
        .bind(op_type)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_proposer(&self, tx: &str, block_num: i64, proposer: &str) -> Result<()> {
        sqlx::query(
            "insert into evm_e_proposer values($1,$2,$3) on conflict(tx,proposer) do update set block_num=$2",
        )
        .bind(tx)
        .bind(block_num)
        .bind(proposer)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_epoch(&self, tx: &str, block_num: i64, epoch: i64) -> Result<()> {
        sqlx::query(
            "insert into evm_e_epoch values($1,$2,$3) on conflict(tx,epoch) do update set block_num=$2",
        )
        .bind(tx)
        .bind(block_num)
        .bind(epoch)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_jailed(
        &self,
        tx: &str,
        block_num: i64,
        validator: &str,
        jailed: bool,
    ) -> Result<()> {
        sqlx::query(
            "insert into evm_e_jailed values($1,$2,$3,$4) on conflict(tx,validator) \
                do update set block_num=$2,jailed=$4",
        )
        .bind(tx)
        .bind(block_num)
        .bind(validator)
        .bind(jailed)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn upsert_punish(
        &self,
        tx: &str,
        block_num: i64,
        voted: Value,
        unvoted: Value,
        byztine: Value,
    ) -> Result<()> {
        sqlx::query(
            "insert into evm_e_punish values($1,$2,$3,$4,$5) on conflict(tx) do update \
                set block_num=$2,voted=$3,unvoted=$4,byztine=$5",
        )
        .bind(tx)
        .bind(block_num)
        .bind(voted)
        .bind(unvoted)
        .bind(byztine)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_update_validator(
        &self,
        tx: &str,
        block_num: i64,
        validator: &str,
        memo: &str,
        rate: BigDecimal,
    ) -> Result<()> {
        sqlx::query(
            "insert into evm_e_update_validator values($1,$2,$3,$4,$5) on conflict(tx,validator) \
                do update set block_num=$2,memo=$4,rate=$5",
        )
        .bind(tx)
        .bind(block_num)
        .bind(validator)
        .bind(memo)
        .bind(rate)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_coinbase_mint(
        &self,
        tx: &str,
        block_num: i64,
        validator: &str,
        delegator: &str,
        pubkey: &str,
        amount: BigDecimal,
    ) -> Result<()> {
        sqlx::query(
            "insert into evm_e_coinbase_mint values($1,$2,$3,$4,$5,$6) on \
                conflict(tx,validator,delegator) do update set block_num=$2,pubkey=$5,amount=$6",
        )
        .bind(tx)
        .bind(block_num)
        .bind(validator)
        .bind(delegator)
        .bind(pubkey)
        .bind(amount)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
