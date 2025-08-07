use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::adapters::idp::ExternalIdProvider;

pub(crate) async fn idp(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match headers.get("x-user") {
        Some(user_id_raw) => {
            let Ok(user_id) = user_id_raw.to_str() else {
                return Err(StatusCode::BAD_REQUEST);
            };

            request
                .extensions_mut()
                .insert(ExternalIdProvider::new(user_id.to_owned()));
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
