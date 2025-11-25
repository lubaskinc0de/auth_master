use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    application::errors::auth::external_web::{CannotFetchUserInfoError, ExternalWebAuthError},
    entities::errors::base::{AppError, ErrorKind},
};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    pub(crate) error_code: String,
    pub(crate) error_description: String,
}

impl<E: AppError + Into<StatusCode>> IntoResponse for ErrorKind<E> {
    fn into_response(self) -> axum::response::Response {
        let error_code = self.get_error_code();
        let error_description = self.get_error_description();
        let json = Json(ErrorResponse {
            error_code,
            error_description,
        });
        let status_code = match self {
            ErrorKind::Expected(error) => {
                tracing::info!(error = ?error, "Handling expected error");
                error.into()
            }
            ErrorKind::Unexpected(e) => {
                tracing::error!(error = %e, "Internal Server Error occured");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        (status_code, json).into_response()
    }
}

impl From<CannotFetchUserInfoError> for StatusCode {
    fn from(val: CannotFetchUserInfoError) -> Self {
        match &val {
            CannotFetchUserInfoError::Unauthorized => StatusCode::UNAUTHORIZED,
            CannotFetchUserInfoError::Unavailable => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

impl From<ExternalWebAuthError> for StatusCode {
    fn from(val: ExternalWebAuthError) -> Self {
        match val {
            ExternalWebAuthError::CannotFetchUserInfo(cannot_fetch_user_info_error) => {
                cannot_fetch_user_info_error.into()
            }
            ExternalWebAuthError::SignInRequired => StatusCode::UNAUTHORIZED,
        }
    }
}
