// src/handlers/user_handler.rs
use crate::error::AppError;
use crate::state::AppState;
use axum::{extract::State, Json};
use serde_json::{json, Value};

// 对应 controller.HealthCheckHandler
pub async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

// 对应 controller.SignUpHandler
pub async fn signup(
    State(state): State<AppState>,
    // Json(payload): Json<SignUpPayload>, // 从请求体中提取数据
) -> Result<Json<Value>, AppError> {
    // 使用 state.db_pool 执行数据库操作
    // let user = sqlx::query("...")
    //     .execute(state.db_pool.as_ref())
    //     .await?;
    Ok(Json(json!({ "message": "User signed up successfully" })))
}

// ... 其他处理器 ...