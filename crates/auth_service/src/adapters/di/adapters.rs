use deadpool_postgres::{Object, Pool};
use froodi::{
    DefaultScope::{App, Request},
    Inject, InstantiatorResult,
    async_impl::RegistryWithSync,
    async_registry, registry,
};

use shared::{
    config::Config,
    db::factory::{get_connection, get_connection_pool},
};

use crate::adapters::{gateway::user::SeaUserGateway, id_gen::V4IdGenerator};

async fn get_connection_async(Inject(pool): Inject<Pool>) -> InstantiatorResult<Object> {
    Ok(get_connection(pool).await)
}

pub(crate) fn create_adapters_registry() -> RegistryWithSync {
    let registry = async_registry! {
        scope(Request) [
            provide(get_connection_async),
            provide(async |Inject(conn): Inject<Object>| Ok(SeaUserGateway {conn})),
        ],
        extend(registry! {
            scope(App) [
                provide(|| Ok(V4IdGenerator {})),
            ],
            scope(Request) [
                provide(|Inject(config): Inject<Config>| Ok(get_connection_pool(&config.db))),
            ]
        }),
    };
    registry
}
