use axum::{Extension, Router, middleware, response::Html, routing::get};

use crate::{
    adapters::{http::middleware::auth::idp, idp::ExternalIdProvider},
    application::common::idp::IdProvider,
};

mod adapters;
mod application;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route_layer(middleware::from_fn(idp));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Extension(idp): Extension<ExternalIdProvider>) -> Html<String> {
    let user_id = idp.get_user_id();

    Html(format!("<h1>Hello, {:?}!</h1>", user_id))
}
