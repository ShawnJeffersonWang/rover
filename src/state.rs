// src/state.rs
use sqlx::PgPool;
use std::sync::Arc;

// 使用 Arc 来安全地在多线程间共享状态
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
    // pub redis_client: Arc<redis::Client>,
    // pub kafka_producer: Arc<rdkafka::producer::FutureProducer>,
    // pub snowflake_generator: Arc<sonyflake::Sonyflake>,
}