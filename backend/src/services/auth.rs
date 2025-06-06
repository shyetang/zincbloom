use crate::repositories::{RoleRepository, UserRepository};
use anyhow::{anyhow, Context, Result};
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::config::AppConfig;
use crate::models::{UserLoginPayload, UserPublic, UserRegistrationPayload};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tracing;

// JWT Claims 结构体，定义 Token 中包含的数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,              // Subject (通常是用户ID)
    pub username: String,         // 用户名
    pub exp: usize,               // Expiration timestamp (Unix时间戳，秒)
    pub roles: Vec<String>,       // 角色名称列表
    pub permissions: Vec<String>, // 用户拥有的权限名称列表
}

// 认证服务结构体
#[derive(Clone)]
pub struct AuthSerVice {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>, // 在注册时分配默认角色，需要 RoleRepository
    jwt_secret: String,                 // 用于签名和验证 JWT 的密钥
    jwt_issuer: String,                 // JWT 签发者
    jwt_audience: String,               // JWT 受众
    jwt_expiry_hours: i64,              // JWT 过期时间 (小时)
}

// 密码哈希辅助函数(使用 argon2id)
fn hash_password(password: &str) -> Result<String> {
    // SaltString 会使用 OsRng 自动生成一个加密安全的盐值
    let salt = SaltString::generate(&mut OsRng);

    // Argon2::default() 会使用推荐的、安全的默认参数。
    let argon2 = Argon2::default();

    // 进行哈希计算
    // .hash_password() 方法接收密码字节和盐，返回一个 PasswordHash 对象
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Argon2 哈希密码失败: {}", e))?;
    //PasswordHash: 这是一个核心的抽象。它代表了一个完整的密码哈希字符串（遵循 PHC 字符串格式），
    // 包含了算法、版本、参数、盐和哈希值本身。这使得验证过程更简单、更不容易出错，
    // 因为我们只需传递整个哈希字符串，验证函数会自动解析出所需的一切。

    // 将 PasswordHash 对象转换为 PHC 格式的字符串，以便存储在数据库中
    // 格式类似于： $argon2id$v=19$m=19456,t=2,p=1$SALT_HERE$HASH_HERE
    Ok(password_hash.to_string())
}

// 密码验证辅助函数
fn verify_password(hash_str: &str, password: &str) -> Result<bool> {
    // 从数据库取出的哈希字符串中解析出所有信息（算法、版本、参数、盐、哈希值）
    let parsed_hash =
        PasswordHash::new(hash_str).map_err(|e| anyhow!("解析密码哈希字符串失败: {}", e))?;

    // 使用 Argon2::default() 实例来验证密码
    // .verify_password() 方法会从 parsed_hash 中自动提取盐和参数来进行比较
    let verification_result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    match verification_result {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false), // 密码不匹配
        Err(e) => Err(anyhow!("密码验证过程中发生错误: {}", e)),  // 其他错误，例如哈希格式不兼容等
    }
}

impl AuthSerVice {
    // 构造函数
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        config: &AppConfig,
    ) -> Self {
        let auth_config = &config.auth;
        Self {
            user_repo,
            role_repo,
            jwt_secret: auth_config.jwt_secret.clone(),
            jwt_issuer: auth_config.jwt_issuer.clone(),
            jwt_audience: auth_config.jwt_audience.clone(),
            jwt_expiry_hours: auth_config.jwt_expiry_hours,
        }
    }

    // 用户注册服务方法
    pub async fn register_user(&self, payload: UserRegistrationPayload) -> Result<UserPublic> {
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
            .create_user(&payload, &hashed_password)
            .await?;

        match self.role_repo.find_by_name("user").await {
            Ok(Some(user_role)) => {
                self.user_repo
                    .assign_roles_to_user(new_user.id, &[user_role.id])
                    .await
                    .context(format!("为用户 '{}' 分配默认角色失败", new_user.username))?;
                tracing::info!("成功为新用户 {} 分配 'user' 角色", new_user.username);
            }
            Ok(None) => {
                tracing::info!("关键配置缺失: 默认的 'user' 角色在数据库中未找到。");
                return Err(anyhow!("服务器内部配置错误，无法完成注册。"));
            }
            Err(e) => return Err(e).context("查找默认 'user' 角色时发生数据库错误"),
        }
        let roles_assigned = self.user_repo.get_user_roles(new_user.id).await?;
        let role_names = roles_assigned.into_iter().map(|r| r.name).collect();

        Ok(UserPublic {
            id: new_user.id,
            username: new_user.username,
            email: new_user.email,
            created_at: new_user.created_at,
            roles: role_names,
        })
    }

    // 用户登录服务方法
    pub async fn login_user(&self, payload: UserLoginPayload) -> Result<(String, UserPublic)> {
        let user = self
            .user_repo
            .find_by_username(&payload.username)
            .await
            .context("登录时查找用户失败")?
            .ok_or_else(|| anyhow!("用户名或密码不正确"))?;

        if !verify_password(&user.hashed_password, &payload.password)? {
            return Err(anyhow!("用户名或密码不正确"));
        }
        // 同时获取用户的角色和权限
        let roles = self.user_repo.get_user_roles(user.id).await?;
        let permissions = self.user_repo.get_user_permissions(user.id).await?;

        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();
        let permission_names: Vec<String> = permissions.into_iter().map(|p| p.name).collect();

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(self.jwt_expiry_hours))
            .expect("创建有效JWT过期时间戳失败")
            .timestamp();

        // 创建包含角色和权限的 Claims
        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.to_string(),
            exp: expiration as usize,
            roles: role_names.clone(),
            permissions: permission_names,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .context("生成 JWT 失败")?;

        let user_public = UserPublic {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            roles: role_names,
        };

        Ok((token, user_public))
    }

    // 验证 Token
    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.jwt_issuer]);
        validation.set_audience(&[&self.jwt_audience]);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            tracing::warn!("Token 验证失败: {}, Token: '{}'", e, token);
            anyhow!("无效或过期的 Token: {}", e)
        })
    }
}
