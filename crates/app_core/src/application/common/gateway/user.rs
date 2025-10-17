use crate::domain::entity::user::User;

pub(crate) trait UserGateway {
    async fn create(&self, user: &User);
}