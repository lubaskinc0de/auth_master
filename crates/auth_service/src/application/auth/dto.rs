use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct AuthResponse {
    pub(crate) user_id: uuid::Uuid,
}
