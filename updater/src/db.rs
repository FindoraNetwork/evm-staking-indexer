use crate::error::Result;
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

    pub async fn insert_validator(
        &self,
        block_num: i64,
        validator: &str,
        pubkey: &str,
        pubkey_type: i32,
        rate: BigDecimal,
        staker: &str,
        power: BigDecimal,
        unbound: BigDecimal,
        punish_rate: BigDecimal,
        begin_block: i64,
        active: bool,
        jailed: bool,
        unjail_time: NaiveDateTime,
        should_vote: i32,
        voted: i32,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO evm_validators VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15) \
                ON CONFLICT(block_num,validator,staker) DO UPDATE SET pubkey=$3,pubkey_type=$4,rate=$5,\
                power=$7,unbound=$8,punish_rate=$9,begin_block=$10,active=$11,jailed=$12,unjail_time=$13,\
                should_vote=$14,voted=$15",
        )
        .bind(block_num)
        .bind(validator)
        .bind(pubkey)
        .bind(pubkey_type)
        .bind(rate)
        .bind(staker)
        .bind(power)
        .bind(unbound)
        .bind(punish_rate)
        .bind(begin_block)
        .bind(active)
        .bind(jailed)
        .bind(unjail_time)
        .bind(should_vote)
        .bind(voted)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_validator_list(&self) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT DISTINCT validator FROM evm_stakes")
            .fetch_all(&self.pool)
            .await?;

        let validators = rows
            .iter()
            .map(|r| r.get("validator"))
            .collect::<Vec<String>>();

        Ok(validators)
    }
}
