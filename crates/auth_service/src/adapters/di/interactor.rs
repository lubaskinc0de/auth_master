use crate::application::{common::id_generator::IdGenerator, user::create::CreateUserCommand};
use froodi::{
    DefaultScope::Request,
    Inject,
    async_impl::RegistryWithSync,
    async_registry,
};

use crate::application::common::gateway::user::UserGateway;

pub(crate) fn create_interactor_registry<Gateway: UserGateway, IdGen: IdGenerator>()
-> RegistryWithSync {
    let registry = async_registry! {
        scope(Request) [
            provide(async |Inject(gateway): Inject<Gateway>, Inject(id_gen): Inject<IdGen>| Ok(CreateUserCommand::new(id_gen, gateway)) ),
        ],
    };
    registry
}
