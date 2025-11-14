use crate::db::factory::DbConfig;
use std::env;

const APP_PREFIX: &str = "APP";
const DEFAULT_SERVER_PORT: usize = 3000;
const DEFAULT_HEALTH_PATH: &str = "/health";
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

    let db_host = env::var(var("DB_HOST")).expect("DB_HOST must be set");
    let db_port = env::var(var("DB_PORT"))
        .expect("DB_PORT must be set")
        .parse::<u16>()
        .expect("DB_PORT: expected integer");
    let db_name = env::var(var("DB_NAME")).expect("DB_NAME must be set");
    let db_user = env::var(var("DB_USER")).expect("DB_USER must be set");
    let db_password = env::var(var("DB_PASSWORD")).expect("DB_PASSWORD must be set");
    let db_url = env::var(var("DATABASE_URL")).expect("DATABASE_URL must be set");

    Config {
        core: CoreConfig {
            server: ServerConfig { server_port },
            health: HealthConfig {
                enabled: health_enabled,
                path: health_path,
            },
        },
        db: DbConfig {
            host: db_host,
            port: db_port,
            db_name,
            username: db_user,
            password: db_password,
            url: db_url,
        },
    }
}
