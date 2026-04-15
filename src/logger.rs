//! 日志模块
//! 
//! 该模块负责初始化和配置 Rust 的 tracing 日志系统
//! 类似于 Java 中的 log4j 或 slf4j，提供结构化的日志输出

// 导入 tracing_subscriber 相关组件
// EnvFilter: 用于根据环境变量过滤日志级别
// SubscriberExt: 为订阅者添加扩展方法
// SubscriberInitExt: 提供初始化订阅者的方法
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// 初始化日志系统
/// 
/// 该函数会：
/// 1. 创建一个 tracing 订阅者注册表
/// 2. 配置日志级别过滤器（优先从环境变量读取，默认 info 级别）
/// 3. 配置日志输出格式（包含文件、行号、线程信息等）
/// 4. 初始化并设置为全局订阅者
/// 
/// # 示例用法
/// ```rust
/// // 在程序启动时调用
/// logger::init();
/// 
/// // 之后可以使用 tracing 宏记录日志
/// tracing::info!("Server started on port 3001");
/// tracing::warn!("Resource usage is high");
/// tracing::error!("Failed to connect to database");
/// ```
pub fn init() {
    // 创建订阅者注册表
    // 注册表是 tracing 系统的核心，负责管理和分发日志事件
    tracing_subscriber::registry()
        // 配置日志级别过滤器
        // 优先尝试从环境变量（如 RUST_LOG）读取日志级别
        // 如果环境变量未设置，则默认使用 "info" 级别
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        // 配置日志输出格式
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)        // 输出日志所在文件
                .with_line_number(true) // 输出日志所在行号
                .with_thread_ids(true)  // 输出线程 ID
                .with_thread_names(true) // 输出线程名称
                .with_target(false)     // 不输出目标（模块路径）
        )
        // 初始化并设置为全局订阅者
        // 之后所有的 tracing 日志都会使用这个配置
        .init();
}