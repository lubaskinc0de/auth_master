use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
use froodi::{
    DefaultScope::{App, Request},
    Inject,
    async_impl::{Container, RegistryBuilder},
    instance,
};

use shared::{
    config::Config,
    db::factory::{get_connection, get_connection_pool},
};

pub(crate) fn create_container(config: Config) -> Container {
    let registry = RegistryBuilder::new()
        .provide(instance(config), App)
        .provide(
            |Inject(config): Inject<Config>| Ok(get_connection_pool(&config.db.url)),
            App,
        )
        .provide_async(
            async |Inject(pool): Inject<Pool<AsyncPgConnection>>| Ok(get_connection(pool).await),
            Request,
        );

    Container::new(registry)
}
