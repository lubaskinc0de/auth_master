use std::sync::Arc;

use crate::{
    application::{
        common::{external_auth::ExternalAuthService, tx_manager::TxManager},
        errors::auth::external_web::ExternalWebAuthError,
    },
    entities::errors::base::ErrorKind,
};
use axum::{
    Json,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use froodi::{Inject, InjectTransient};
use shared::config::external_auth_service::ExternalAuthConfig;

use crate::application::{
    auth::external_web::ExternalWebAuth,
    common::{
        gateway::{external_user_id::ExternalUserIdGateway, user::UserGateway},
        id_generator::IdGenerator,
    },
};

pub(crate) async fn external_web_auth<
    UGateway: UserGateway,
    IdGen: IdGenerator,
    AuthService: ExternalAuthService,
    ExternalUIdGateway: ExternalUserIdGateway,
    TxM: TxManager,
>(
    Inject(interactor): Inject<
        ExternalWebAuth<IdGen, UGateway, AuthService, ExternalUIdGateway, TxM>,
    >,
    InjectTransient(config): InjectTransient<Arc<ExternalAuthConfig>>,
) -> Response {
    tracing::info!("GET /");
    let res = interactor.execute().await;

    if let Err(e) = res {
        match &e {
            ErrorKind::Expected(err) => match &err {
                ExternalWebAuthError::SignInRequired => {
                    let mut resp = e.into_response();
                    resp.headers_mut().insert(
                        "WWW-Authenticate",
                        HeaderValue::from_str(&config.endpoint_sign_in)
                            .expect("Invalid header value"),
                    );
                    resp
                }
                _ => e.into_response(),
            },
            ErrorKind::Unexpected(_) => e.into_response(),
        }
    } else if let Ok(auth_response) = res {
        let mut resp = res
            .map(|result| (StatusCode::CREATED, Json(result).into_response()))
            .into_response();
        resp.headers_mut().insert(
            "X-Auth-User",
            HeaderValue::from_str(&auth_response.user_id.to_string())
                .expect("Invalid header value"),
        );
        resp
    } else {
        unreachable!()
    }
}
