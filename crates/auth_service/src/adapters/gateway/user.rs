use std::sync::Arc;

use deadpool_postgres::Object;
use sea_query::{PostgresQueryBuilder, Query};

use crate::{
    adapters::models::user::DbUser, application::common::gateway::user::UserGateway,
    domain::entity::user::User,
};

pub(crate) struct SeaUserGateway {
    pub(crate) conn: Arc<Object>,
}

impl UserGateway for SeaUserGateway {
    async fn create(&self, user: &User) {
        let q = Query::insert()
            .into_table(DbUser::Table)
            .columns([
                DbUser::Id,
                DbUser::Email,
                DbUser::Username,
                DbUser::IsBanned,
                DbUser::CreatedAt,
            ])
            .values([
                user.id.into(),
                user.email.clone().into(),
                user.username.clone().into(),
                user.is_banned.into(),
                user.created_at.into(),
            ])
            .unwrap()
            .take();
        let sql = q.to_string(PostgresQueryBuilder);
        self.conn
            .execute(&sql, &[])
            .await
            .expect("Create user query failed");  // TODO: handle errors
    }
}
