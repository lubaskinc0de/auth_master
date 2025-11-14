use std::sync::Arc;

use serde::{Deserialize, Serialize};

use uuid::Uuid;
use validator::Validate;

use crate::{
    application::common::{gateway::user::UserGateway, id_generator::IdGenerator},
    domain::entity::user::User,
};

#[derive(Validate, Deserialize)]
pub(crate) struct CreateUserForm {
    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 100))]
    username: String,
}

#[derive(Serialize)]
pub(crate) struct CreateUserResponse {
    pub user_id: Uuid,
}

pub(crate) struct CreateUserCommand<IdGen, Gateway> {
    id_generator: Arc<IdGen>,
    gateway: Arc<Gateway>,
}

impl<IdGen: IdGenerator, Gateway: UserGateway> CreateUserCommand<IdGen, Gateway> {
    pub fn new(id_generator: Arc<IdGen>, gateway: Arc<Gateway>) -> Self {
        Self {
            id_generator,
            gateway,
        }
    }

    pub async fn execute(&self, data: CreateUserForm) -> CreateUserResponse {
        let user_id = self.id_generator.generate();
        let user = User::new(user_id, data.email, data.username);

        self.gateway.create(&user).await;
        CreateUserResponse { user_id }
    }
}
