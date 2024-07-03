use crate::error;
use error::Result;
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
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

    pub async fn upsert_block(
        &self,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
    ) -> Result<()> {
        sqlx::query("insert into evm_blocks values($1,$2,$3) on conflict(block_id) do update set block_num=$2,tm=$3")
            .bind(block_id)
            .bind(block_num)
            .bind(tm)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn upsert_tx(
        &self,
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
    ) -> Result<()> {
        sqlx::query("insert into evm_txs values($1,$2,$3,$4) on conflict(tx_id) do update set block_id=$2,block_num=$3,tm=$4")
            .bind(tx_id)
            .bind(block_id)
            .bind(block_num)
            .bind(tm)
            .execute(&self.pool)
            .await?;
        Ok(())
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
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
        validator: &str,
        pubkey: &str,
        ty: i32,
        staker: &str,
        amount: BigDecimal,
        memo: Value,
        rate: BigDecimal,
    ) -> Result<()> {
        sqlx::query("INSERT INTO evm_stakes VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) ON \
            CONFLICT(tx_id,validator,staker) DO UPDATE SET block_id=$2,block_num=$3,tm=$4,pubkey=$6,ty=$7,amount=$9,memo=$10,rate=$11"
        )
            .bind(tx_id)
            .bind(block_id)
            .bind(block_num)
            .bind(tm)
            .bind(validator)
            .bind(pubkey)
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
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
        validator: &str,
        delegator: &str,
        amount: BigDecimal,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO evm_delegations VALUES($1,$2,$3,$4,$5,$6,$7) ON \
                CONFLICT(tx_id,validator,delegator) DO UPDATE SET block_id=$2,block_num=$3,tm=$4,amount=$7",
        )
            .bind(tx_id)
            .bind(block_id)
            .bind(block_num)
            .bind(tm)
            .bind(validator)
            .bind(delegator)
            .bind(amount)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn upsert_undelegation(
        &self,
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
        idx: i32,
        validator: &str,
        delegator: &str,
        unlock_time: i64,
        amount: BigDecimal,
        op_type: i32,
    ) -> Result<()> {
        sqlx::query("INSERT INTO evm_undelegations VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON \
            CONFLICT(tx_id,validator,delegator) DO UPDATE SET block_id=$2,block_num=$3,tm=$4,idx=$5,unlock_time=$8,amount=$9,op_type=$10"
        )
            .bind(tx_id)
            .bind(block_id)
            .bind(block_num)
            .bind(tm)
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

    pub async fn upsert_jailed(
        &self,
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
        validator: &str,
        jailed: bool,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO evm_jailed VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(tx_id,validator) \
                DO UPDATE SET block_id=$2,block_num=$3,tm=$4,jailed=$6",
        )
        .bind(tx_id)
        .bind(block_id)
        .bind(block_num)
        .bind(tm)
        .bind(validator)
        .bind(jailed)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_punish(
        &self,
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
        voted: Value,
        unvoted: Value,
        byztine: Value,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO evm_punish VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(tx_id) DO UPDATE \
                SET block_id=$2,block_num=$3,tm=$4,voted=$5,unvoted=$6,byztine=$7",
        )
        .bind(tx_id)
        .bind(block_id)
        .bind(block_num)
        .bind(tm)
        .bind(voted)
        .bind(unvoted)
        .bind(byztine)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_update_validator(
        &self,
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
        validator: &str,
        memo: Value,
        rate: BigDecimal,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO evm_update_validator values($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(tx_id,validator) \
                DO UPDATE SET block_id=$2,block_num=$3,tm=$4,memo=$6,rate=$7",
        )
            .bind(tx_id)
            .bind(block_id)
            .bind(block_num)
            .bind(tm)
            .bind(validator)
            .bind(memo)
            .bind(rate)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn upsert_coinbase_mint(
        &self,
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        tm: NaiveDateTime,
        validator: &str,
        delegator: &str,
        pubkey: &str,
        amount: BigDecimal,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO evm_coinbase_mint VALUES($1,$2,$3,$4,$5,$6,$7,$8) ON \
                CONFLICT(tx_id,validator,delegator) DO UPDATE SET block_id=$2,block_num=$3,tm=$4,pubkey=$7,amount=$8",
        )
            .bind(tx_id)
            .bind(block_id)
            .bind(block_num)
            .bind(tm)
            .bind(validator)
            .bind(delegator)
            .bind(pubkey)
            .bind(amount)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn upsert_evm_receipt(
        &self,
        tx_id: &str,
        block_id: &str,
        block_num: i64,
        from: &str,
        to: &str,
        tm: NaiveDateTime,
        value: Value,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO evm_receipts VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(tx_id) \
            DO UPDATE SET block_id=$2,block_num=$3,from_addr=$4,to_addr=$5,tm=$6,value=$7",
        )
        .bind(tx_id)
        .bind(block_id)
        .bind(block_num)
        .bind(from)
        .bind(to)
        .bind(tm)
        .bind(value)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
