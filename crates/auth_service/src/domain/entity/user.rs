use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) is_banned: bool,
    pub(crate) created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid, email: String, username: String) -> Self {
        Self {
            id,
            email,
            username,
            is_banned: false,
            created_at: Utc::now(),
        }
    }
}
