use froodi::{async_impl::Container, async_registry};
use shared::config::Config;

use crate::adapters::{
    di::{
        adapters::create_adapters_registry, config::create_config_registry,
        interactor::create_interactor_registry,
    },
    gateway::user::SeaUserGateway,
    id_gen::V4IdGenerator,
};

pub(crate) fn create_container(config: Config) -> Container {
    let config = create_config_registry(config);
    let adapters = create_adapters_registry();
    let interactors = create_interactor_registry::<SeaUserGateway, V4IdGenerator>();
    Container::new(async_registry! { extend(adapters, interactors, config),  })
}
