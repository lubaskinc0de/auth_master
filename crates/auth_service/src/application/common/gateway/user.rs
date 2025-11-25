use uuid::Uuid;

use crate::entities::{
    entity::user::User,
    errors::base::{Infallible, InfallibleVoid},
    shared::ThreadSafe,
};

pub(crate) trait UserGateway: ThreadSafe {
    async fn create(&self, user: &User) -> InfallibleVoid;
    async fn get(&self, user_id: Uuid) -> Infallible<Option<User>>;
}
