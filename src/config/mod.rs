mod database;
mod server;

use anyhow::Context;
use config::{Config, FileFormat};
use serde::Deserialize;
use std::sync::LazyLock;

pub use database::DatabaseConfig;
pub use server::ServerConfig;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initialize config"));

// Debug: 自动实现 fmt::Debug，用于调试打印
// Deserialize: 自动实现 serde 反序列化，用于从配置文件读取
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

impl AppConfig {
    // 类似 SpringBoot 的 @ConfigurationProperties 风格
    // 依次从 application.yml 和环境变量 APP_* 读取配置
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(config::File::with_name("application").format(FileFormat::Yaml))
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .context("Failed to load config")?
            .try_deserialize()
            .context("Failed to deserialize config")
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
