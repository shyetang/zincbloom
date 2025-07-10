use crate::config::AppConfig;
use crate::dtos::admin::{UserLoginPayload, UserRegistrationPayload};
use crate::dtos::auth::ResetPasswordPayload;
use crate::models::{User, UserPublic};
use crate::repositories::{
    LoginAttemptRepository, OneTimeTokenRepository, RoleRepository, UserRepository,
};
use crate::services::EmailService;
use crate::utils::{hash_password, validate_password_strength, verify_password};
use anyhow::{Context, Result, anyhow};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing;
use uuid::Uuid;

// --- 用于保证token唯一性的静态原子计数器 ---
static TOKEN_COUNTER: AtomicUsize = AtomicUsize::new(0);

// JWT Claims 结构体，定义 Token 中包含的数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,              // Subject (通常是用户ID)
    pub username: String,         // 用户名
    pub exp: usize,               // Expiration timestamp (Unix时间戳，秒)
    pub jti: String,              // JWT ID，确保唯一性
    pub roles: Vec<String>,       // 角色名称列表
    pub permissions: Vec<String>, // 用户拥有的权限名称列表
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginTokens {
    pub access_token: String,
    pub refresh_token: String,
}

// 认证服务结构体
#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
    login_attempt_repo: Arc<dyn LoginAttemptRepository>,
    one_time_token_repo: Arc<dyn OneTimeTokenRepository>,
    email_service: Arc<EmailService>,
    access_token_secret: String,      // 用于签名和验证 token 的密钥
    jwt_issuer: String,               // JWT 签发者
    jwt_audience: String,             // JWT 受众
    access_token_expiry_minutes: i64, // Access token 的有效期（分钟）
    refresh_token_expiry_days: i64,   // Refresh token 的有效期（天）
    max_login_failures: u32,          // 最大尝试登录失败次数
    lockout_duration_seconds: i64,    // 锁定登录时间
}

// 生成一个长而随机的字符串作为 Refresh Token
fn generate_refresh_token() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(64) // 64位长度
        .map(char::from)
        .collect()
}

// 使用 blake3 对 Token 进行哈希，以便存入数据库
fn hash_token(token: &str) -> String {
    let hash = blake3::hash(token.as_bytes());
    // to_hex() 返回一个特殊的 Hex an dDisplay 类型，将其转换为 String
    hash.to_hex().to_string()
}

// 生成一次性令牌
fn generate_one_time_token() -> String {
    // 1. 获取一个唯一的计数值
    let counter = TOKEN_COUNTER.fetch_add(1, Ordering::Relaxed);

    // 2. 获取高精度时间戳
    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0);

    // 3. 生成随机数部分
    let random_part: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(24) // 24位长度
        .map(char::from)
        .collect();

    // 拼接 UUID 的一部分来保证在并发测试中的唯一性
    format!("{}-{}-{}", timestamp, counter, random_part)
}

impl AuthService {
    // 构造函数
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        login_attempt_repo: Arc<dyn LoginAttemptRepository>,
        one_time_token_repo: Arc<dyn OneTimeTokenRepository>,
        email_service: Arc<EmailService>,
        config: &AppConfig,
    ) -> Self {
        Self {
            user_repo,
            role_repo,
            login_attempt_repo,
            one_time_token_repo,
            email_service,
            access_token_secret: config.auth.jwt_secret.clone(),
            jwt_issuer: config.auth.jwt_issuer.clone(),
            jwt_audience: config.auth.jwt_audience.clone(),
            access_token_expiry_minutes: config.auth.access_token_expiry_minutes,
            refresh_token_expiry_days: config.auth.refresh_token_expiry_days,
            max_login_failures: config.auth.max_login_failures,
            lockout_duration_seconds: config.auth.lockout_duration_seconds,
        }
    }

    // 用户注册服务方法(在事务中进行)
    pub async fn register_user(&self, payload: UserRegistrationPayload) -> Result<UserPublic> {
        // 验证密码策略
        validate_password_strength(&payload.password).context("用户注册时密码强度验证失败")?;

        // 开启事务
        let mut tx = self
            .user_repo
            .get_pool()
            .begin()
            .await
            .context("开启注册事务失败")?;

        if self
            .user_repo
            .find_by_username(&payload.username)
            .await?
            .is_some()
        {
            return Err(anyhow!("用户名 '{}' 已被占用", payload.username));
        }
        if self
            .user_repo
            .find_by_email(&payload.email)
            .await?
            .is_some()
        {
            return Err(anyhow!("邮箱 '{}' 已被注册", payload.email));
        }
        let hashed_password = hash_password(&payload.password).context("注册时，哈希密码失败")?;
        let new_user = self
            .user_repo
            .create_in_tx(&mut tx, &payload, &hashed_password)
            .await?;

        match self.role_repo.find_by_name("author").await {
            Ok(Some(author_role)) => {
                self.user_repo
                    .assign_roles_to_user_in_tx(&mut tx, new_user.id, &[author_role.id])
                    .await
                    .context(format!("为用户 '{}' 分配默认角色失败", new_user.username))?;
                tracing::info!("成功为新用户 {} 分配 'author' 角色", new_user.username);
            }
            Ok(None) => {
                tracing::info!("关键配置缺失: 默认的 'author' 角色在数据库中未找到。");
                return Err(anyhow!("服务器内部配置错误，无法完成注册。"));
            }
            Err(e) => return Err(e).context("查找默认 'author' 角色时发生数据库错误"),
        }
        let roles_assigned = self.user_repo.get_user_roles(new_user.id).await?;
        let role_names = roles_assigned.into_iter().map(|r| r.name).collect();

        // --- 发送邮箱验证 ---
        let verification_token = generate_one_time_token();
        let token_hash = hash_token(&verification_token);
        let expires_at = Utc::now() + Duration::hours(24);

        self.one_time_token_repo
            .store_token_in_tx(
                &mut tx,
                new_user.id,
                &token_hash,
                "email_verification",
                expires_at,
            )
            .await?;

        // 准备邮件内容
        let verification_link = format!(
            "http://localhost:3000/verify-email?token={}",
            verification_token
        ); // URL应来自配置
        let email_subject = "欢迎来到我的博客 - 请验证您的邮箱地址";
        let email_body = format!(
            "<p>您好, {},</p><p>感谢您的注册！请点击下面的链接来验证您的邮箱地址：</p><p><a href=\"{}\">验证邮箱</a></p><p>此链接将在24小时后失效。</p>",
            new_user.username, verification_link
        );

        // 提交事务（在发送邮件之前提交，确保邮件失败不会影响用户创建）
        tx.commit().await.context("提交注册事务失败")?;

        // 发送邮件（如果失败，记录警告但不阻止注册）
        if let Err(e) = self.email_service
            .send_email(&new_user.email, email_subject, &email_body)
            .await {
            tracing::warn!("注册邮件发送失败，用户 {} 已创建但未发送验证邮件: {}", new_user.username, e);
        }

        tracing::info!("用户 {} 注册成功并提交事务", new_user.username);

        Ok(UserPublic {
            id: new_user.id,
            username: new_user.username,
            email: new_user.email,
            created_at: new_user.created_at,
            roles: role_names,
        })
    }

    // 处理邮箱验证
    pub async fn verify_email(&self, token: &str) -> Result<()> {
        let token_hash = hash_token(token);

        // 查找并消费令牌
        let user_id = self
            .one_time_token_repo
            .find_and_consume_token(&token_hash, "email_verification")
            .await?
            .ok_or_else(|| anyhow!("邮箱验证失败：无效或已过期的令牌"))?;

        // 将用户的email_verified_at 字段更新为当前时间
        self.user_repo.mark_email_as_verified(user_id).await?;

        tracing::info!("用户 {} 的邮箱已成功验证", user_id);
        Ok(())
    }

    // 用户登录服务方法
    pub async fn login_user(&self, payload: UserLoginPayload) -> Result<(LoginTokens, UserPublic)> {
        self.check_lockout_status(&payload.username).await?;

        // 验证用户凭证
        let user = match self.user_repo.find_by_username(&payload.username).await? {
            Some(user) => user,
            None => {
                self.handle_login_failure(&payload.username).await?;
                return Err(anyhow!("用户名或密码不正确"));
            }
        };

        if !verify_password(&user.hashed_password, &payload.password)? {
            self.handle_login_failure(&payload.username).await?;
            return Err(anyhow!("用户名或密码不正确"));
        }

        self.handle_login_success(&payload.username).await?;

        // 生成一套 Token
        let tokens = self.generate_and_store_tokens(&user).await?;

        // 准备返回给客户端的用户公开信息
        let user_roles = self.user_repo.get_user_roles(user.id).await?;
        let user_public = UserPublic {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            roles: user_roles.into_iter().map(|r| r.name).collect(),
        };

        Ok((tokens, user_public))
    }

    // logout
    pub async fn logout(&self, refresh_token: &str) -> Result<()> {
        let token_hash = hash_token(refresh_token);
        self.user_repo
            .delete_refresh_token(&token_hash)
            .await
            .context("注销时删除 Refresh Token 失败")
    }

    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<LoginTokens> {
        let token_hash = hash_token(refresh_token);

        // 验证 Refresh Token 是否有效，并获取关联用户
        let user = self
            .user_repo
            .find_user_by_refresh_token(&token_hash)
            .await
            .context("验证 Refresh Token 时发生数据库错误")?
            .ok_or_else(|| anyhow!("无效的 Refresh Token"))?;

        // 立即使当前 Refresh Token 失效
        self.user_repo
            .delete_refresh_token(&token_hash)
            .await
            .context("注销时删除 Refresh Token 失败")?;

        // 为用户生成一套新的 Token
        let tokens = self.generate_and_store_tokens(&user).await?;

        tracing::info!("成功为用户 {} 刷新了 Token", user.username);
        Ok(tokens)
    }

    // 生成 Token
    async fn generate_and_store_tokens(&self, user: &User) -> Result<LoginTokens> {
        // 生成 Access Token
        let roles = self.user_repo.get_user_roles(user.id).await?;
        let permissions = self.user_repo.get_user_permissions(user.id).await?;
        let access_token_exp = Utc::now() + Duration::minutes(self.access_token_expiry_minutes);

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            exp: access_token_exp.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            roles: roles.into_iter().map(|r| r.name).collect(),
            permissions: permissions.into_iter().map(|p| p.name).collect(),
        };
        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_token_secret.as_ref()),
        )
        .context("生成 Access Token 失败")?;

        // 生成 Refresh Token，并存储
        let refresh_token = generate_refresh_token();
        let refresh_token_hash = hash_token(&refresh_token);
        let refresh_token_exp = Utc::now() + Duration::days(self.refresh_token_expiry_days);
        self.user_repo
            .store_refresh_token(user.id, &refresh_token_hash, refresh_token_exp)
            .await
            .context("存储 Refresh Token 时发生数据库错误")?;

        Ok(LoginTokens {
            access_token,
            refresh_token,
        })
    }

    // 验证 Token
    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.jwt_issuer]);
        validation.set_audience(&[&self.jwt_audience]);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.access_token_secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            tracing::warn!("Token 验证失败: {}, Token: '{}'", e, token);
            anyhow!("无效或过期的 Token: {}", e)
        })
    }

    // 检查账户是否被锁定
    async fn check_lockout_status(&self, username: &str) -> Result<()> {
        if let Some(attempt) = self.login_attempt_repo.get_by_username(username).await? {
            if let Some(lockout_expires_at) = attempt.lockout_expires_at {
                if Utc::now() < lockout_expires_at {
                    let remaining = lockout_expires_at - Utc::now();
                    return Err(anyhow!(
                        "账户已被锁定，请在 {} 分 {} 秒后重试",
                        remaining.num_minutes(),
                        remaining.num_seconds() % 60
                    ));
                }
            }
        }
        Ok(())
    }

    // 处理登录失败的方法
    async fn handle_login_failure(&self, username: &str) -> Result<()> {
        let failure_count = self.login_attempt_repo.record_failure(username).await?;

        if failure_count as u32 >= self.max_login_failures {
            let expires_at = Utc::now() + Duration::seconds(self.lockout_duration_seconds);
            self.login_attempt_repo
                .set_lockout(username, expires_at)
                .await?;
            tracing::warn!("用户 {} 因登录失败次数过多已被锁定", username);
        }
        Ok(())
    }
    // 处理登录成功
    async fn handle_login_success(&self, username: &str) -> Result<()> {
        self.login_attempt_repo.clear_for_username(username).await
    }

    // 请求密码重置
    pub async fn request_password_reset(&self, email: &str) -> Result<()> {
        // 根据邮箱查找用户
        let user = match self.user_repo.find_by_email(email).await? {
            // **重要安全策略**: 如果邮箱不存在，我们不应该告诉客户端“该邮箱未注册”。
            // 而是应该静默地成功返回，就像邮件已经发送了一样。
            // 这可以防止攻击者通过这个接口来探测哪些邮箱是已注册用户（用户名枚举攻击）。
            None => {
                tracing::warn!("收到一个不存在的邮箱 {} 的密码重置请求，静默处理。", email);
                return Ok(());
            }
            Some(user) => user,
        };

        // 生成并存储一次性重置token
        let reset_token = generate_one_time_token();
        let token_hash = hash_token(&reset_token);
        let expires_at = Utc::now() + Duration::minutes(15);

        self.one_time_token_repo
            .store_token(user.id, "password_reset", &token_hash, expires_at)
            .await?;

        // 发送密码重置邮件
        let reset_link = format!("http://localhost:3000/reset-password?token={}", reset_token); // URL应来自配置
        let email_subject = "密码重置请求权";
        let email_body = format!(
            "<p>您好, {},</p><p>我们收到了一个为您的账户重置密码的请求。请点击下面的链接来设置新密码：</p><p><a href=\"{}\">重置密码</a></p><p>如果您没有请求重置密码，请忽略此邮件。此链接将在30分钟后失效。</p>",
            user.username, reset_link
        );

        self.email_service
            .send_email(&user.email, email_subject, &email_body)
            .await
    }

    // 重置密码
    pub async fn reset_password(&self, payload: &ResetPasswordPayload) -> Result<()> {
        // 基础验证
        if payload.new_password != payload.confirm_password {
            return Err(anyhow!("新密码与确认密码不匹配"));
        }
        validate_password_strength(&payload.new_password)?;

        // 验证并消费重置令牌
        let token_hash = hash_token(&payload.token);
        let user_id = self
            .one_time_token_repo
            .find_and_consume_token(&token_hash, "password_reset")
            .await?
            .ok_or_else(|| anyhow!("密码重置失败：无效、已过期或已使用的令牌"))?;

        // 哈希新密码并更新到数据库
        let new_hashed_password = hash_password(&payload.new_password)?;
        self.user_repo
            .update_password(user_id, &new_hashed_password)
            .await?;
        // 密码重置后，让该用户所有其他设备上的会话都失效
        self.user_repo
            .delete_all_refresh_token_for_user(user_id)
            .await?;

        tracing::info!("用户 {} 已成功重置密码，并撤销了所有旧会话。", user_id);
        Ok(())
    }
}
