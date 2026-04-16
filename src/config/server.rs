use serde::Deserialize;

// Debug: 自动实现 fmt::Debug，用于调试打印 (类似 Java 的 toString())
// Deserialize: 自动实现 serde 反序列化，用于从配置文件读取 (类似 Jackson 的 @JsonProperty)
// 对应 Java 代码:
// @Data @ToString
// @JsonProperties
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: Option<u16>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}
