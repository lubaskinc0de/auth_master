use serde::Deserialize;

use uuid::Uuid;
use validator::Validate;

use crate::{
    application::common::{gateway::user::UserGateway, id_generator::IdGenerator},
    domain::entity::user::{Gender, User},
};

#[derive(Debug, Validate, Deserialize)]
pub(crate) struct CreateUserForm {
    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 100))]
    username: String,

    #[validate(length(min = 1, max = 100))]
    first_name: String,

    #[validate(length(min = 1, max = 100))]
    last_name: String,

    #[validate(range(min = 14, max = 99))]
    age: usize,

    gender: Gender,
}

pub(crate) struct CreateUserResponse {
    pub user_id: Uuid,
}

pub(crate) struct CreateUserCommand<IdGen: IdGenerator, Gateway: UserGateway> {
    id_generator: IdGen,
    gateway: Gateway,
}

impl<IdGen: IdGenerator, Gateway: UserGateway> CreateUserCommand<IdGen, Gateway> {
    pub fn new(id_generator: IdGen, gateway: Gateway) -> Self {
        Self {
            id_generator,
            gateway,
        }
    }

    pub async fn execute(&self, data: CreateUserForm) -> CreateUserResponse {
        let user_id = self.id_generator.generate();
        let user = User::new(
            user_id,
            data.email,
            data.username,
            data.first_name,
            data.age,
            data.last_name,
            data.gender,
        );

        self.gateway.create(&user).await;

        CreateUserResponse { user_id }
    }
}
