use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
pub trait OneTimeTokenRepository: Send + Sync {
    // 存储一个新的一次性令牌
    async fn store_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        token_type: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()>;
    async fn store_token_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        token_hash: &str,
        token_type: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()>;

    // 查找并消费（删除）一个令牌。如果成功，返回关联的用户ID。
    // 这是核心方法，必须保证原子性。
    async fn find_and_consume_token(
        &self,
        token_hash: &str,
        token_type: &str,
    ) -> Result<Option<Uuid>>;
}

#[derive(Clone)]
pub struct PostgresOneTimeTokenRepository {
    pool: PgPool,
}
impl PostgresOneTimeTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl OneTimeTokenRepository for PostgresOneTimeTokenRepository {
    async fn store_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        token_type: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query!(
            "insert into one_time_tokens (token_hash, user_id, token_type, expires_at) values ($1,$2,$3,$4)",
            token_hash,
            user_id,
            token_type,
            expires_at
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn store_token_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        token_hash: &str,
        token_type: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query!(
            "insert into one_time_tokens (token_hash, user_id, token_type, expires_at) values ($1,$2,$3,$4)",
            token_hash,
            user_id,
            token_type,
            expires_at
        )
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    async fn find_and_consume_token(
        &self,
        token_hash: &str,
        token_type: &str,
    ) -> Result<Option<Uuid>> {
        // 在单个事务中，执行先查找后删除，保证原子性
        let mut tx = self.pool.begin().await.context("开启数据库事务失败")?;
        // 查找有效(未过期)的token
        let record = sqlx::query!(
            "select user_id from one_time_tokens where token_hash=$1 and token_type = $2 and expires_at > now()",
            token_hash,
            token_type
        )
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(record) = record {
            // 如果找到，立即删除它，确保它只能被使用一次
            sqlx::query!(
                "delete from one_time_tokens where token_hash = $1",
                token_hash
            )
            .execute(&mut *tx)
            .await?;

            // 提交事务
            tx.commit().await.context("提交消费令牌的事务失败")?;
            // 返回找到的user_id
            Ok(Some(record.user_id))
        } else {
            // 如果没有找到，回滚事务（虽然这里没什么可回滚的，但这是好习惯）
            tx.rollback().await.ok();
            Ok(None)
        }
    }
}
