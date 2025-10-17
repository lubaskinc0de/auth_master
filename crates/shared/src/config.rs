use crate::db::factory::DbConfig;
use std::env;

const APP_PREFIX: &'static str = "APP";
const DEFAULT_SERVER_PORT: usize = 3000;
const DEFAULT_HEALTH_PATH: &'static str = "/health";
const DEFAULT_HEALTH_ENABLED: bool = true;

#[derive(Clone)]
pub struct ServerConfig {
    pub server_port: usize,
}

#[derive(Clone)]
pub struct HealthConfig {
    pub enabled: bool,
    pub path: String,
}

#[derive(Clone)]
pub struct CoreConfig {
    pub server: ServerConfig,
    pub health: HealthConfig,
}

#[derive(Clone)]
pub struct Config {
    pub core: CoreConfig,
    pub db: DbConfig,
}

fn var(name: &'static str) -> String {
    format!("{APP_PREFIX}_{name}")
}

pub fn from_env() -> Config {
    let server_port = env::var(var("SERVER_PORT"))
        .unwrap_or_else(|_| DEFAULT_SERVER_PORT.to_string())
        .parse::<usize>()
        .unwrap();

    let health_enabled = env::var(var("HEALTH_ENABLED"))
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(DEFAULT_HEALTH_ENABLED);

    let health_path =
        env::var(var("HEALTH_PATH")).unwrap_or_else(|_| DEFAULT_HEALTH_PATH.to_string());
    let db_url = env::var(var("DATABASE_URL")).expect("DATABASE_URL must be set");

    Config {
        core: CoreConfig {
            server: ServerConfig { server_port },
            health: HealthConfig {
                enabled: health_enabled,
                path: health_path,
            },
        },
        db: DbConfig { url: db_url },
    }
}
