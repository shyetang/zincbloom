use anyhow::{Context, Result};
use config::{Config as ConfigRs, Environment, File};
use serde::Deserialize;

// 邮件服务相关配置
#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub from_address: String,
}
// 认证相关配置结构体
#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,    // JWT 签名密钥
    pub jwt_issuer: String,    // JWT 签发者
    pub jwt_audience: String,  // JWT 受众
    pub access_token_expiry_minutes: i64, // Access Token 过期时间 分钟
    pub refresh_token_expiry_days: i64,   // Refresh Token 过期时间 天
    pub max_login_failures: u32,    // 允许的最大登录失败次数
    pub lockout_duration_seconds: i64,  // 账户锁定时长 秒
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub email: EmailConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        // 尝试加载 .env 文件（忽略错误，如果文件不存在）
        let _ = dotenvy::dotenv();

        let cfg_builder = ConfigRs::builder()
            // 添加默认值
            .set_default("server.port", 8080)?
            .set_default("server.host", "127.0.0.1")?
            // 为认证配置添加默认值
            // !! 警告: 这里的 jwt_secret 只是一个占位符，绝对不能在生产环境中使用 !!
            // !! 必须通过环境变量 (APP_AUTH__JWT_SECRET) 来覆盖它 !!
            .set_default("auth.jwt_secret", "default_secret_that_must_be_changed")?
            .set_default("auth.jwt_issuer", "my_blog_app")?
            .set_default("auth.jwt_audience", "my_blog_app_users")?
            .set_default("auth.access_token_expiry_minutes", 15)?
            .set_default("auth.refresh_token_expiry_days", 7)?
            .set_default("auth.max_login_failures", 5)? // 默认允许失败5次
            .set_default("auth.lockout_duration_seconds", 900)? // 默认锁定 15 分钟 (900秒)
            // 邮件相关默认值
            .set_default("email.smtp_host","localhost")?
            .set_default("email.smtp_port",1025)?
            .set_default("email.smtp_user","")?
            .set_default("email.smtp_pass","")?
            .set_default("email.from_address","np-reply@localhost.com")?
            // .set_default(...)? // 其他默认值

            // 从环境变量加载配置
            // 前缀为 "APP"，分隔符为 "__"
            // 例如: APP_SERVER__PORT=9000 会覆盖 server.port
            //       APP_DATABASE__URL=postgres://... 会设置 database.url
            .add_source(Environment::with_prefix("APP").separator("__"))
            // (可选) 从配置文件加载
            .add_source(File::with_name("config/default").required(false)) // 例如 config/default.toml
            // .add_source(File::with_name("config/production").required(false)) // 例如 config/production.toml
            ;

        // 构建配置实例
        let settings = cfg_builder
            .build()
            .context("构建配置实例失败 (来自环境变量/文件)")?; // 添加上下文
        // 将配置反序列化到 AppConfig 结构体中
        let app_config = settings
            .try_deserialize::<AppConfig>()
            .context("反序列化配置到 AppConfig 结构体失败")?; // 添加上下文

        Ok(app_config) // 返回包含配置的 Result::Ok
    }
}
