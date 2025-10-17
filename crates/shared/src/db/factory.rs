use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Object, Pool};

#[derive(Clone)]
pub struct DbConfig {
    pub url: String,
}

pub fn get_connection_pool(db_url: &str) -> Pool<AsyncPgConnection> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(config)
        .build()
        .expect("Failed to create db pool");
    pool
}

pub async fn get_connection(pool: Arc<Pool<AsyncPgConnection>>) -> Object<AsyncPgConnection> {
    let conn = pool.get().await.expect("Failed to get connection");
    conn
}
