use crate::application::{
    auth::external_web::ExternalWebAuth,
    common::{
        external_auth::ExternalAuthService, gateway::external_user_id::ExternalUserIdGateway,
        id_generator::IdGenerator, tx_manager::TxManager,
    },
};
use froodi::{
    DefaultScope::Request, Inject, InstantiatorResult, async_impl::RegistryWithSync, async_registry,
};

use crate::application::common::gateway::user::UserGateway;

async fn get_external_web_auth_interactor<
    UGateway: UserGateway,
    IdGen: IdGenerator,
    AuthService: ExternalAuthService,
    ExternalUIdGateway: ExternalUserIdGateway,
    TxM: TxManager,
>(
    Inject(user_gateway): Inject<UGateway>,
    Inject(id_gen): Inject<IdGen>,
    Inject(auth_service): Inject<AuthService>,
    Inject(external_user_id_gateway): Inject<ExternalUIdGateway>,
    Inject(tx_manager): Inject<TxM>,
) -> InstantiatorResult<ExternalWebAuth<IdGen, UGateway, AuthService, ExternalUIdGateway, TxM>> {
    Ok(ExternalWebAuth::new(
        id_gen,
        user_gateway,
        auth_service,
        external_user_id_gateway,
        tx_manager,
    ))
}

pub(crate) fn create_interactor_registry<
    UGateway: UserGateway,
    IdGen: IdGenerator,
    AuthService: ExternalAuthService,
    ExternalUIdGateway: ExternalUserIdGateway,
    TxM: TxManager,
>() -> RegistryWithSync {
    let registry = async_registry! {
        scope(Request) [
            provide(
            get_external_web_auth_interactor::<
                UGateway,
                IdGen,
                AuthService,
                ExternalUIdGateway,
                TxM
            >),
        ],
    };
    registry
}
