use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub(crate) struct AuthResponse {
    pub(crate) user_id: uuid::Uuid,
}
