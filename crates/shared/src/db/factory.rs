use deadpool_postgres::{Config, ManagerConfig, Object, Pool, RecyclingMethod, Runtime};
use std::sync::Arc;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

pub fn get_connection_pool(config: &DbConfig) -> Pool {
    let mut cfg = Config::new();
    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.db_name.clone());
    cfg.user = Some(config.username.clone());
    cfg.port = Some(config.port);
    cfg.password = Some(config.password.clone());

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}

pub async fn get_connection(pool: Arc<Pool>) -> Object {
    pool.get().await.expect("Failed to get connection")
}
