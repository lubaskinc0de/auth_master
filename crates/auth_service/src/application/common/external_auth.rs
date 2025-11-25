use crate::{
    application::errors::auth::external_web::CannotFetchUserInfoError,
    entities::{errors::base::Failable, shared::ThreadSafe},
};

pub(crate) trait ExternalAuthService: ThreadSafe {
    async fn get_external_id(&self) -> Failable<String, CannotFetchUserInfoError>;
}
