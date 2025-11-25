use thiserror::Error;

use crate::entities::errors::base::AppError;

#[derive(Error, Debug)]
pub(crate) enum CannotFetchUserInfoError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Unavailable")]
    Unavailable,
}

#[derive(Error, Debug)]
pub(crate) enum ExternalWebAuthError {
    #[error(transparent)]
    CannotFetchUserInfo(CannotFetchUserInfoError),
    #[error("SignInRequired")]
    SignInRequired,
}

impl AppError for ExternalWebAuthError {
    fn get_error_code(&self) -> String {
        match &self {
            ExternalWebAuthError::CannotFetchUserInfo(_) => "CANNOT_FETCH_USER_INFO".to_string(),
            ExternalWebAuthError::SignInRequired => "SIGN_IN_REQUIRED".to_string(),
        }
    }
    fn get_error_description(&self) -> String {
        self.to_string()
    }
}
