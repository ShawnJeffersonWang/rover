// src/main.rs
mod config;
mod error;
mod middleware;
mod routes;
mod state;
mod handlers;

use crate::state::AppState;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载配置
    let settings = config::load_config().expect("Failed to load configuration.");

    // 2. 初始化日志 (Tracing)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.log.level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3. 初始化数据库连接池
    let db_pool = PgPoolOptions::new()
        .max_connections(settings.postgres.max_connections)
        .connect(&settings.postgres.dsn())
        .await
        .expect("Failed to create Postgres pool.");
    tracing::info!("PostgreSQL connection pool initialized.");

    // 在这里初始化其他服务 (Redis, Kafka, Snowflake) ...

    // 4. 创建应用共享状态
    let app_state = AppState {
        db_pool: Arc::new(db_pool),
        jwt_secret: Arc::new(settings.jwt.secret), // 从配置初始化
        // ...
    };

    // 5. 创建路由
    let app = routes::create_router(app_state)
        .layer(TraceLayer::new_for_http()); // 添加HTTP请求日志中间件

    // 6. 启动服务器
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", settings.server.port)).await?;
    tracing::info!("Server listening on port {}", settings.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server has shut down.");
    Ok(())
}

// 优雅关机信号处理器
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { tracing::info!("Received Ctrl+C, starting graceful shutdown..."); },
        _ = terminate => { tracing::info!("Received terminate signal, starting graceful shutdown..."); },
    }
}