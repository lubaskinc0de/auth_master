use axum::{Json, Router, http::StatusCode, routing::get};
use axum_valid::Valid;
use froodi::{Inject, axum::setup_async_default};
use shared::config;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{
        di::container::create_container, gateway::user::SeaUserGateway, id_gen::V4IdGenerator,
    },
    application::{
        common::{gateway::user::UserGateway, id_generator::IdGenerator},
        user::create::{CreateUserCommand, CreateUserForm, CreateUserResponse},
    },
    presentation as presentation_layer,
};

mod adapters;
mod application;
mod domain;
mod presentation;

#[tokio::main]
async fn main() {
    let config = config::from_env();
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_file(true)
                .with_line_number(true)
                .with_target(true),
        )
        .init();
    let container = create_container(config.clone());

    let mut app = setup_async_default(
        Router::new().route("/", get(create_user::<V4IdGenerator, SeaUserGateway>)),
        container,
    );

    if config.core.health.enabled {
        app = app.route(
            &config.core.health.path,
            get(presentation_layer::health_check),
        );
    }

    let addr = format!("0.0.0.0:{}", config.core.server.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_user<IdGen: IdGenerator, Gateway: UserGateway>(
    Inject(interactor): Inject<CreateUserCommand<IdGen, Gateway>>,
    data: Valid<Json<CreateUserForm>>,
) -> (StatusCode, Json<CreateUserResponse>) {
    tracing::info!("GET /");
    (
        StatusCode::CREATED,
        Json(interactor.execute(data.into_inner().0).await),
    )
}
