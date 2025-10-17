use axum::{Router, response::Html, routing::get};
use froodi::{Inject, axum::setup_async_default};
use shared::config;

use crate::{
    adapters::{di::container::create_container, idp::ExternalIdProvider},
    application::common::idp::IdProvider,
    presentation as presentation_layer,
};

mod adapters;
mod application;
mod domain;
mod presentation;

#[tokio::main]
async fn main() {
    let config = config::from_env();
    let container = create_container(config.clone());

    let mut app = setup_async_default(
        Router::new().route("/", get(handler::<ExternalIdProvider>)),
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

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler<I: IdProvider>(Inject(idp): Inject<I>) -> Html<String> {
    let user_id = idp.get_user_id();

    Html(format!("<h1>Hello, {:?}!</h1>", user_id))
}
