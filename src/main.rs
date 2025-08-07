use axum::{Extension, Router, middleware, response::Html, routing::get};

use crate::{
    adapters::{config, http::middleware::auth::idp, idp::ExternalIdProvider},
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

    let mut app = Router::new()
        .route("/", get(handler))
        .route_layer(middleware::from_fn(idp));

    if config.health.enabled {
        app = app.route(&config.health.path, get(presentation_layer::health_check));
    }

    let addr = format!("0.0.0.0:{}", config.server.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Extension(idp): Extension<ExternalIdProvider>) -> Html<String> {
    let user_id = idp.get_user_id();

    Html(format!("<h1>Hello, {:?}!</h1>", user_id))
}
