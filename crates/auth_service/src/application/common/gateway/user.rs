use crate::domain::entity::user::User;

pub(crate) trait UserGateway: Send + Sync + 'static {
    async fn create(&self, user: &User);
}