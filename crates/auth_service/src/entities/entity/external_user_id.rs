use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

#[derive(Clone, FromSql, ToSql, Debug)]
#[postgres(name = "external_id_source")]
pub(crate) enum ExternalIdSource {
    #[postgres(name = "WEB")]
    Web,
}

pub(crate) struct ExternalUserId {
    pub(crate) user_id: Uuid,
    pub(crate) external_id: String,
    pub(crate) source: ExternalIdSource,
    pub(crate) created_at: DateTime<Utc>,
}

impl ExternalUserId {
    pub fn new(user_id: Uuid, external_id: String, source: ExternalIdSource) -> Self {
        Self {
            user_id,
            external_id,
            source,
            created_at: Utc::now(),
        }
    }
}
