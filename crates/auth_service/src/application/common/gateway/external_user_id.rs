use crate::entities::{
    entity::external_user_id::ExternalUserId,
    errors::base::{Infallible, InfallibleVoid},
    shared::ThreadSafe,
};

pub(crate) trait ExternalUserIdGateway: ThreadSafe {
    async fn create(&self, external_user_id: &ExternalUserId) -> InfallibleVoid;
    async fn get(&self, external_id: &str) -> Infallible<Option<ExternalUserId>>;
}
