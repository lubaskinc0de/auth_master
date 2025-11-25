use std::sync::Arc;

use crate::{
    adapters::models::external_user_id::ExternalUserId,
    application::common::gateway::external_user_id::ExternalUserIdGateway,
    entities::{
        entity::{self, external_user_id::ExternalIdSource},
        errors::base::{Infallible, InfallibleVoid},
        shared::ThreadSafe,
    },
    unexpected_err,
};
use deadpool_postgres::Object;
use sea_query::{Expr, ExprTrait, Iden, PostgresQueryBuilder, Query};

pub(crate) struct SeaExternalUserIdGateway {
    pub(crate) conn: Arc<Object>,
}

impl ExternalUserIdGateway for SeaExternalUserIdGateway {
    async fn create(
        &self,
        external_user_id: &entity::external_user_id::ExternalUserId,
    ) -> InfallibleVoid {
        let q = Query::insert()
            .into_table(ExternalUserId::Table)
            .columns([
                ExternalUserId::UserId,
                ExternalUserId::ExternalId,
                ExternalUserId::Source,
                ExternalUserId::CreatedAt,
            ])
            .values([
                external_user_id.user_id.into(),
                external_user_id.external_id.clone().into(),
                external_user_id.source.clone().into(),
                external_user_id.created_at.into(),
            ])
            .unwrap()
            .take();
        let sql = q.to_string(PostgresQueryBuilder);
        unexpected_err!(self.conn.execute(&sql, &[]).await);
        Ok(())
    }

    async fn get(
        &self,
        external_id: &str,
    ) -> Infallible<Option<entity::external_user_id::ExternalUserId>> {
        let q = Query::select()
            .from(ExternalUserId::Table)
            .columns([
                ExternalUserId::UserId,
                ExternalUserId::ExternalId,
                ExternalUserId::Source,
                ExternalUserId::CreatedAt,
            ])
            .and_where(Expr::col(ExternalUserId::ExternalId).eq(external_id))
            .to_owned();
        let sql = q.to_string(PostgresQueryBuilder);
        Ok(
            if let Some(result) = unexpected_err!(self.conn.query_opt(&sql, &[]).await) {
                Some(entity::external_user_id::ExternalUserId {
                    user_id: result.get(ExternalUserId::UserId.to_string().as_str()),
                    source: result
                        .get::<&str, ExternalIdSource>(ExternalUserId::Source.to_string().as_str()),
                    created_at: result.get(ExternalUserId::CreatedAt.to_string().as_str()),
                    external_id: result.get(ExternalUserId::ExternalId.to_string().as_str()),
                })
            } else {
                None
            },
        )
    }
}

impl ThreadSafe for SeaExternalUserIdGateway {}
