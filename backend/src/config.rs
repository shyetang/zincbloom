use anyhow::{Context, Result};
use config::{Config as ConfigRs, Environment, File};
use serde::Deserialize;

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
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        // 尝试加载 .env 文件（忽略错误，如果文件不存在）
        let _ = dotenvy::dotenv();

        let cfg_builder = ConfigRs::builder()
            // 可以添加默认值
            .set_default("server.port", 8080)?
            .set_default("server.host", "127.0.0.1")?
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
