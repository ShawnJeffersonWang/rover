// src/middleware.rs
use crate::error::AppError;
use crate::state::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
// 建议导入 tracing
use tracing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// JWT 认证中间件
pub async fn jwt_auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "));

    if let Some(token_str) = token {
        // --- 问题在这里 ---
        // 修改前:
        // let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_ref());

        // 修改后:
        let decoding_key = DecodingKey::from_secret(state.jwt_secret.as_bytes()); // 使用 .as_bytes() 而不是 .as_ref()

        let token_data = decode::<Claims>(token_str, &decoding_key, &Validation::default())
            .map_err(|err| {
                tracing::warn!("JWT decode error: {}", err);
                AppError::AuthError
            })?;

        req.extensions_mut().insert(Arc::new(token_data.claims));

        Ok(next.run(req).await)
    } else {
        Err(AppError::AuthError)
    }
}