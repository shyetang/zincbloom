use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

// 用于返回登录尝试状态的结构体
#[derive(Debug, Default)]
pub struct LoginAttempt {
    pub failure_count: i32,
    pub lockout_expires_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait LoginAttemptRepository: Send + Sync {
    // 获取一个用户的登录尝试状态
    async fn get_by_username(&self, username: &str) -> Result<Option<LoginAttempt>>;
    // 记录一次失败的尝试，并返回当前的失败次数
    async fn record_failure(&self, username: &str) -> Result<i32>;
    // 为用户设置锁定时间
    async fn set_lockout(&self, username: &str, expires_at: DateTime<Utc>) -> Result<()>;
    // 清除一个用户的所有登录失败记录
    async fn clear_for_username(&self, username: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct PostgresLoginAttemptRepository {
    pool: PgPool,
}

impl PostgresLoginAttemptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LoginAttemptRepository for PostgresLoginAttemptRepository {
    async fn get_by_username(&self, username: &str) -> Result<Option<LoginAttempt>> {
        let record = sqlx::query_as!(
            LoginAttempt,
            "select failure_count,lockout_expires_at from login_attempts where username = $1",
            username
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(record)
    }

    async fn record_failure(&self, username: &str) -> Result<i32> {
        // 使用 UPSERT 原子性地更新失败次数，并返回新的次数
        let record = sqlx::query!(
            r#"
            insert into login_attempts (username, failure_count) values ($1,1)
            on conflict (username) do update 
            set failure_count = login_attempts.failure_count + 1,last_attempt_at = NOW()
            returning failure_count
            "#,
            username
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(record.failure_count)
    }

    async fn set_lockout(&self, username: &str, expires_at: DateTime<Utc>) -> Result<()> {
        sqlx::query!(
            "update login_attempts set lockout_expires_at = $1 where username = $2",
            expires_at,
            username
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn clear_for_username(&self, username: &str) -> Result<()> {
        sqlx::query!(
            "delete from login_attempts where username = $1",
            username
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}