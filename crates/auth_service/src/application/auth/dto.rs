use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct AuthResponse {
    pub user_id: uuid::Uuid,
}
