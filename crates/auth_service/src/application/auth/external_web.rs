use std::sync::Arc;

use crate::{
    application::{
        auth::dto::AuthResponse,
        common::{
            external_auth::ExternalAuthService,
            gateway::{external_user_id::ExternalUserIdGateway, user::UserGateway},
            id_generator::IdGenerator,
            tx_manager::TxManager,
        },
        errors::auth::external_web::{CannotFetchUserInfoError, ExternalWebAuthError},
    },
    entities::{
        entity::{
            external_user_id::{ExternalIdSource, ExternalUserId},
            user::User,
        },
        errors::base::{ErrorKind, Failable, Infallible},
    },
};

pub(crate) struct ExternalWebAuth<IdGen, UserGateway, AuthService, ExternalUIdGateway, TxManager> {
    id_generator: Arc<IdGen>,
    user_gateway: Arc<UserGateway>,
    auth_service: Arc<AuthService>,
    tx_manager: Arc<TxManager>,
    external_user_id_gateway: Arc<ExternalUIdGateway>,
}

impl<
    IdGen: IdGenerator,
    UGateway: UserGateway,
    AuthService: ExternalAuthService,
    ExternalUIdGateway: ExternalUserIdGateway,
    TxM: TxManager,
> ExternalWebAuth<IdGen, UGateway, AuthService, ExternalUIdGateway, TxM>
{
    pub(crate) fn new(
        id_generator: Arc<IdGen>,
        user_gateway: Arc<UGateway>,
        auth_service: Arc<AuthService>,
        external_user_id_gateway: Arc<ExternalUIdGateway>,
        tx_manager: Arc<TxM>,
    ) -> Self {
        Self {
            id_generator,
            user_gateway,
            auth_service,
            external_user_id_gateway,
            tx_manager,
        }
    }

    pub(crate) async fn execute(&self) -> Failable<AuthResponse, ExternalWebAuthError> {
        tracing::debug!("External web auth called");
        self.tx_manager.begin().await?;

        let external_user_id_query = self.auth_service.get_external_id().await;
        let external_user_id_value = external_user_id_query.map_err(|e| match e {
            ErrorKind::Unexpected(e) => ErrorKind::Unexpected(e),
            ErrorKind::Expected(e) => ErrorKind::Expected(match e {
                CannotFetchUserInfoError::Unauthorized => {
                    tracing::debug!("Cannot fetch user info: Unauthorized. Sign in required");
                    ExternalWebAuthError::SignInRequired
                }
                CannotFetchUserInfoError::Unavailable => {
                    tracing::warn!("External web auth service unavailable");
                    ExternalWebAuthError::CannotFetchUserInfo(e)
                }
            }),
        })?;
        tracing::debug!(
            external_user_id = external_user_id_value,
            "User authorized by external service"
        );

        if let Some(db_external_user_id) = self
            .external_user_id_gateway
            .get(&external_user_id_value)
            .await?
        {
            tracing::debug!("Db external user id found");
            if let Some(user) = self.user_gateway.get(db_external_user_id.user_id).await? {
                tracing::info!(user_id = %user.id, "User found");
                return Ok(AuthResponse { user_id: user.id });
            } else {
                tracing::warn!("User not found by external_user_id.user_id");
                let user = self.create_user(external_user_id_value).await?;
                return Ok(AuthResponse { user_id: user.id });
            }
        } else {
            let user = self.create_user(external_user_id_value).await?;
            Ok(AuthResponse { user_id: user.id })
        }
    }

    async fn create_user(&self, external_user_id: String) -> Infallible<User> {
        let user = User::new(self.id_generator.generate());
        let external_id = ExternalUserId::new(user.id, external_user_id, ExternalIdSource::Web);

        self.user_gateway.create(&user).await?;
        self.external_user_id_gateway.create(&external_id).await?;
        self.tx_manager.commit().await?;
        tracing::info!("User created");
        Ok(user)
    }
}
