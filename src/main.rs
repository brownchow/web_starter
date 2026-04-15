use axum::{Router, routing};
use tokio::net::TcpListener; // 使用 Tokio 的 TcpListener 以支持异步操作

#[tokio::main] // Tokio 运行时属性标记
async fn main() {
    // 创建路由器并定义根路径的 GET 请求处理程序
    let router = Router::new()
        .route("/", routing::get(async || "Hello, Rust!"));

    // 绑定到本地地址和端口
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    // 启动 HTTP 服务器
    axum::serve(listener, router).await.unwrap();
}
