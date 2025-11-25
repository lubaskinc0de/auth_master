use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) is_banned: bool,
    pub(crate) created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            is_banned: false,
            created_at: Utc::now(),
        }
    }
}
