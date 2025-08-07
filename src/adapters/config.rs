use std::env;

const APP_PREFIX: &'static str = "CRUDIK";
const DEFAULT_SERVER_PORT: usize = 3000;

pub(crate) struct ServerConfig {
    pub server_port: usize,
}

pub(crate) struct HealthConfig {
    pub enabled: bool,
    pub path: String,
}

pub(crate) struct Config {
    pub server: ServerConfig,
    pub health: HealthConfig,
}

fn var(name: &'static str) -> String {
    format!("{APP_PREFIX}_{name}")
}

pub(crate) fn from_env() -> Config {
    let server_port = env::var(var("SERVER_PORT"))
        .unwrap_or_else(|_| DEFAULT_SERVER_PORT.to_string())
        .parse::<usize>()
        .unwrap();

    let health_enabled = env::var(var("HEALTH_ENABLED"))
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let health_path = env::var(var("HEALTH_PATH")).unwrap_or_else(|_| "/health".to_string());

    Config {
        server: ServerConfig { server_port },
        health: HealthConfig {
            enabled: health_enabled,
            path: health_path,
        },
    }
}
