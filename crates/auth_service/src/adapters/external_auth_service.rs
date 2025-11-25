use axum::http::request::Parts;
use reqwest::StatusCode;
use serde::Deserialize;
use shared::config::external_auth_service::ExternalAuthConfig;
use std::sync::Arc;

use crate::{
    application::{
        common::external_auth::ExternalAuthService,
        errors::auth::external_web::CannotFetchUserInfoError,
    },
    entities::{
        errors::base::{ErrorKind, Failable},
        shared::ThreadSafe,
    },
    unexpected_err,
};

#[derive(Deserialize, Debug)]
struct UserInfoResponse {
    user: String,
    #[allow(dead_code)]
    email: String,
}

pub(crate) struct OAuth2ProxyService {
    client: Arc<reqwest::Client>,
    config: Arc<ExternalAuthConfig>,
    request: Arc<Parts>,
}

impl OAuth2ProxyService {
    pub(crate) fn new(
        client: Arc<reqwest::Client>,
        config: Arc<ExternalAuthConfig>,
        request: Arc<Parts>,
    ) -> Self {
        Self {
            client,
            config,
            request,
        }
    }
}

impl ExternalAuthService for OAuth2ProxyService {
    async fn get_external_id(&self) -> Failable<String, CannotFetchUserInfoError> {
        let url = &self.config.endpoint_userinfo;
        tracing::debug!(url = url, "GET request to oauth2-proxy");
        let request = unexpected_err!(
            self.client
                .get(url)
                .headers(self.request.headers.clone())
                .build()
        );
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|_| ErrorKind::Expected(CannotFetchUserInfoError::Unavailable))?;

        tracing::debug!("Received response from oauth2-proxy");
        match response.status() {
            StatusCode::OK => {
                tracing::debug!("Received OK response from ouath2-proxy");
                let response_text = unexpected_err!(response.text().await);
                let user_info =
                    unexpected_err!(serde_json::from_str::<UserInfoResponse>(&response_text));
                tracing::info!(user_info = %user_info.user, "Oauth2-proxy authenticated user");
                return Ok(user_info.user);
            }
            StatusCode::UNAUTHORIZED => {
                tracing::info!("Received UNAUTHORIZED response from oauth2-proxy");
                Err(ErrorKind::Expected(CannotFetchUserInfoError::Unauthorized))
            }
            code => {
                tracing::warn!(status_code = %code, "Received unknown response from oauth2-proxy");
                Err(ErrorKind::Expected(CannotFetchUserInfoError::Unavailable))
            }
        }
    }
}

impl ThreadSafe for OAuth2ProxyService {}
