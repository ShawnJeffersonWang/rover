// src/routes.rs
use crate::{
    handlers::user_handler,
    middleware,
    state::AppState,
};
use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub fn create_router(state: AppState) -> Router {
    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // === 定义需要 JWT 认证的路由 ===
    // 这种结构将所有受保护的路由集中在一起，便于管理和应用中间件
    let protected_routes = Router::new()
        // 用户模块的认证路由
        .route("/user/info", get(user_handler::health_check)) // 示例: /user/info
        // 社区模块的认证路由
        // .route("/community-post/create", post(community_handler::create_post)) // 示例
        // ... 其他所有需要JWT认证的路由都定义在这里 ...
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::jwt_auth,
        )); // 对所有受保护的路由应用一次JWT中间件

    // === 定义公共路由（无需认证） ===
    let public_routes = Router::new()
        .route("/health", get(user_handler::health_check))
        // 用户模块的公开路由
        .route("/user/signup", post(user_handler::signup))
        // 社区模块的公开路由
        .route("/community-post/posts/guest", get(user_handler::health_check));
    // ... 其他所有公开路由 ...

    // === 组合最终的路由 ===
    Router::new()
        .merge(public_routes) // 合并公共路由
        .merge(protected_routes) // 合并受保护的路由
        .layer(cors) // 对所有路由应用CORS中间件
        .with_state(state) // 注入应用状态
}