use std::sync::Arc;

use axum::http::request::Parts;
use deadpool_postgres::{Object, Pool};
use froodi::{
    DefaultScope::{App, Request},
    Inject, InjectTransient, InstantiatorResult,
    async_impl::RegistryWithSync,
    async_registry, registry,
};

use shared::{
    config::{Config, external_auth_service::ExternalAuthConfig},
    db::factory::{get_connection, get_connection_pool},
};

use crate::adapters::{
    external_auth_service::OAuth2ProxyService,
    gateway::{external_user_id::SeaExternalUserIdGateway, user::SeaUserGateway},
    id_gen::V4IdGenerator,
    tx_manager::{PgTxManager, finalize_tx_manager},
};

async fn get_connection_async(Inject(pool): Inject<Pool>) -> InstantiatorResult<Object> {
    Ok(get_connection(pool).await)
}

pub(crate) fn create_adapters_registry() -> RegistryWithSync {
    let registry = async_registry! {
        scope(Request) [
            provide(get_connection_async),
            provide(async |Inject(conn): Inject<Object>| Ok(SeaUserGateway {conn})),
            provide(async |Inject(conn): Inject<Object>| Ok(SeaExternalUserIdGateway {conn})),
            provide(async |Inject(conn): Inject<Object>| Ok(PgTxManager::new(conn)), finalizer = finalize_tx_manager),
            provide(
                    async |
                    Inject(client): Inject<reqwest::Client>,
                    InjectTransient(config): InjectTransient<Arc<ExternalAuthConfig>>,
                    Inject(request): Inject<Parts>
                    |
                    Ok(OAuth2ProxyService::new(client, config, request))
                )
        ],
        extend(registry! {
            scope(App) [
                provide(|| Ok(V4IdGenerator {})),
                provide(|| Ok(reqwest::Client::new())),
            ],
            scope(Request) [
                provide(|Inject(config): Inject<Config>| Ok(get_connection_pool(&config.db))),
            ]
        }),
    };
    registry
}
