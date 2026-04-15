// 引入 axum 框架的路由相关组件
// Router: 路由器，用于定义 URL 路由规则
// routing: 包含 get/post 等 HTTP 方法处理函数
use axum::{Router, routing};

// 引入 Tokio 的 TcpListener 用于网络监听
// Tokio 是 Rust 的异步运行时，类似于 Java 的 Netty 或 Node.js 的事件循环
use tokio::net::TcpListener;

// #[tokio::main] 是一个宏（属性宏），它会自动帮我们：
// 1. 创建一个 Tokio 运行时（Runtime）
// 2. 将 async fn main 包装在运行时中执行
// 
// 类比 Java:
// - 类似于 @SpringBootApplication 注解
// - 或者类似于在 main 里调用 SpringApplication.run()
// - 没有它，async main 函数无法直接运行，因为 Rust 标准库没有内置异步运行时
#[tokio::main]
async fn main() {
    // ============================================
    // 第一步：创建路由（Router）
    // ============================================
    // Router::new() 创建一个新的路由器实例
    // .route() 定义路由规则：
    //   - 第一个参数 "/" 是 URL 路径
    //   - 第二个参数是处理函数，这里用 routing::get() 指定处理 GET 请求
    // 
    // async || "Hello, Rust!" 是一个异步闭包（匿名函数）
    // 当访问 http://localhost:3001/ 时，返回 "Hello, Rust!"
    let router = Router::new()
        .route("/", routing::get(async || "Hello, Rust!"));

    // ============================================
    // 第二步：绑定网络端口
    // ============================================
    // TcpListener::bind() 创建一个 TCP 监听器，绑定到 0.0.0.0:3001
    // - 0.0.0.0 表示监听所有网络接口（本机、局域网、外网都能访问）
    // - 3001 是端口号
    //
    // .await 关键字：等待 bind 操作完成（异步等待，不阻塞线程）
    // - bind 是 IO 操作，可能需要时间
    // - .await 会暂停当前函数，让出线程去处理其他任务
    // - 等 bind 完成后，再继续执行
    //
    // .unwrap()：如果 bind 失败（如端口被占用），程序会 panic 并退出
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    // ============================================
    // 第三步：启动 HTTP 服务器
    // ============================================
    // axum::serve() 启动服务器，接收两个参数：
    //   - listener: TCP 监听器（负责接收网络连接）
    //   - router: 路由器（负责处理 HTTP 请求）
    //
    // .await 让服务器一直运行，监听传入的请求
    // 这是一个无限循环，程序会在这里一直运行，直到手动停止（Ctrl+C）
    //
    // .unwrap() 处理启动错误（如权限不足等）
    axum::serve(listener, router).await.unwrap();
}
