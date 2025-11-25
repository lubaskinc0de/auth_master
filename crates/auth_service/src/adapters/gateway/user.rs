use crate::entities::errors::base::Infallible;
use std::sync::Arc;

use deadpool_postgres::Object;
use sea_query::{Expr, ExprTrait, Iden, PostgresQueryBuilder, Query};
use uuid::Uuid;

use crate::{
    adapters::models::user::User,
    application::common::gateway::user::UserGateway,
    entities::{
        entity::{self},
        errors::base::InfallibleVoid,
        shared::ThreadSafe,
    },
    unexpected_err,
};

pub(crate) struct SeaUserGateway {
    pub(crate) conn: Arc<Object>,
}

impl UserGateway for SeaUserGateway {
    async fn create(&self, user: &entity::user::User) -> InfallibleVoid {
        let q = Query::insert()
            .into_table(User::Table)
            .columns([User::Id, User::IsBanned, User::CreatedAt])
            .values([
                user.id.into(),
                user.is_banned.into(),
                user.created_at.into(),
            ])
            .unwrap()
            .take();
        let sql = q.to_string(PostgresQueryBuilder);
        unexpected_err!(self.conn.execute(&sql, &[]).await);
        Ok(())
    }

    async fn get(&self, user_id: uuid::Uuid) -> Infallible<Option<entity::user::User>> {
        let q = Query::select()
            .from(User::Table)
            .columns([User::Id, User::IsBanned, User::CreatedAt])
            .and_where(Expr::col(User::Id).eq(user_id))
            .to_owned();
        let sql = q.to_string(PostgresQueryBuilder);
        Ok(
            if let Some(result) = unexpected_err!(self.conn.query_opt(&sql, &[]).await) {
                Some(entity::user::User {
                    id: result.get::<&str, Uuid>(User::Id.to_string().as_str()),
                    is_banned: result.get(User::IsBanned.to_string().as_str()),
                    created_at: result.get(User::CreatedAt.to_string().as_str()),
                })
            } else {
                None
            },
        )
    }
}

impl ThreadSafe for SeaUserGateway {}
