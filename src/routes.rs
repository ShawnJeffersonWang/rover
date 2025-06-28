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
        .allow_origin(Any) // 对应 AllowOrigins: []string{"*"}
        .allow_methods(Any) // 对应 AllowMethods
        .allow_headers(Any); // 对应 AllowHeaders

    // 用户模块的认证路由组
    let user_auth_routes = Router::new()
        .route("/info", get(user_handler::health_check)) // 示例
        // ... 其他需要JWT的路由 ...
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::jwt_auth,
        ));

    // 社区模块的认证路由组
    let community_auth_routes = Router::new()
        // ...
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::jwt_auth,
        ));

    Router::new()
        .route("/health", get(user_handler::health_check))
        // 用户模块
        .nest("/user", Router::new()
            .route("/signup", post(user_handler::signup))
            // ... 其他公开路由 ...
            .merge(user_auth_routes) // 合并需要认证的路由
        )
        // 社区模块
        .nest("/community-post", Router::new()
            .route("/posts/guest", get(user_handler::health_check))
            // ... 其他公开路由 ...
            .merge(community_auth_routes) // 合并需要认证的路由
        )
        // ... 其他模块 ...
        .layer(cors) // 应用CORS中间件
        .with_state(state) // 注入应用状态
}