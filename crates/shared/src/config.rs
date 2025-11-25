use crate::{config::external_auth_service::ExternalAuthConfig, db::factory::DbConfig};
use std::{env, str::FromStr, sync::Arc};

pub mod external_auth_service;

const APP_PREFIX: &str = "AUTHMASTER";
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
pub struct AuthServiceConfig {
    pub server: ServerConfig,
    pub health: HealthConfig,
}

#[derive(Clone)]
pub struct Config {
    pub auth_service: Arc<AuthServiceConfig>,
    pub db: Arc<DbConfig>,
    pub external_auth: Arc<ExternalAuthConfig>,
}

fn get_env_var(key: &str) -> String {
    env::var(&format!("{APP_PREFIX}_{key}")).unwrap_or_else(|_| panic!("{key} must be set"))
}

fn get_optional_env<T: ToString + FromStr>(key: &str, default: T) -> T {
    env::var(&format!("{APP_PREFIX}_{key}"))
        .unwrap_or_else(|_| default.to_string())
        .parse()
        .unwrap_or(default)
}

pub fn from_env() -> Config {
    Config {
        auth_service: Arc::new(AuthServiceConfig {
            server: ServerConfig {
                server_port: get_optional_env("SERVER_PORT", DEFAULT_SERVER_PORT),
            },
            health: HealthConfig {
                enabled: get_optional_env("HEALTH_ENABLED", DEFAULT_HEALTH_ENABLED),
                path: get_optional_env("HEALTH_PATH", DEFAULT_HEALTH_PATH.to_string()),
            },
        }),
        db: Arc::new(DbConfig {
            host: get_env_var("DB_HOST"),
            port: get_env_var("DB_PORT")
                .parse()
                .expect("DB_PORT: expected integer"),
            db_name: get_env_var("DB_NAME"),
            username: get_env_var("DB_USER"),
            password: get_env_var("DB_PASSWORD"),
            url: get_env_var("DATABASE_URL"),
        }),
        external_auth: Arc::new(ExternalAuthConfig {
            endpoint_userinfo: get_env_var("WEB_ENDPOINT_USERINFO"),
            endpoint_sign_in: get_env_var("WEB_ENDPOINT_SIGN_IN"),
        }),
    }
}
