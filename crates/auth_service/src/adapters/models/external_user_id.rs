use sea_query::{Iden, Value};

use crate::entities::entity::external_user_id::ExternalIdSource;

#[derive(Iden)]
pub enum ExternalUserId {
    Table,
    UserId,
    ExternalId,
    Source,
    CreatedAt,
}

impl From<ExternalIdSource> for Value {
    fn from(val: ExternalIdSource) -> Self {
        Value::String(match val {
            ExternalIdSource::Web => Some("WEB".to_string()),
        })
    }
}
