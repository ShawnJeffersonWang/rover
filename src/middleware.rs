// src/middleware.rs
use crate::error::AppError;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
// ... 其他 use ...

// 这是一个JWT认证中间件的骨架
pub async fn jwt_auth<B>(
    // state: State<AppState>, // 如果需要访问数据库
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(token) = auth_header.and_then(|s| s.strip_prefix("Bearer ")) {
        // 在这里验证 token...
        // let claims = decode_token(token, &state.jwt_secret)?;
        // 可以将用户信息放入请求扩展中，供后续处理器使用
        // req.extensions_mut().insert(claims);
        println!("Token validated: {}", token); // 示例
        Ok(next.run(req).await)
    } else {
        Err(AppError::AuthError)
    }
}