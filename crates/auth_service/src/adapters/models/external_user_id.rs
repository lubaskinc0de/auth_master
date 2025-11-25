use sea_query::{Iden, Value};
use tokio_postgres::types::FromSql;

use crate::entities::entity::external_user_id::ExternalIdSource;

#[derive(Iden)]
pub enum ExternalUserId {
    Table,
    UserId,
    ExternalId,
    Source,
    CreatedAt,
}

impl Into<Value> for ExternalIdSource {
    fn into(self) -> Value {
        Value::String(match self {
            ExternalIdSource::Web => Some("WEB".to_string()),
        })
    }
}
