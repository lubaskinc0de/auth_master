use axum::{Router, routing::get};
use froodi::axum::setup_async_default;
use shared::config;

use crate::{
    adapters::{
        di::container::create_container,
        external_auth_service::OAuth2ProxyService,
        gateway::{external_user_id::SeaExternalUserIdGateway, user::SeaUserGateway},
        id_gen::V4IdGenerator,
        logging::init_logging,
        tx_manager::PgTxManager,
    },
    presentation::{self as presentation_layer, user::external_web_auth},
};

mod adapters;
mod application;
mod entities;
mod presentation;

#[tokio::main]
async fn main() {
    let config = config::from_env();
    init_logging();

    let container = create_container(config.clone());

    let mut app = setup_async_default(
        Router::new().route(
            "/",
            get(external_web_auth::<
                SeaUserGateway,
                V4IdGenerator,
                OAuth2ProxyService,
                SeaExternalUserIdGateway,
                PgTxManager,
            >),
        ),
        container,
    );

    if config.auth_service.health.enabled {
        app = app.route(
            &config.auth_service.health.path,
            get(presentation_layer::health_check),
        );
    }

    let addr = format!("0.0.0.0:{}", config.auth_service.server.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
