use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) enum Gender {
    MAN,
    WOMAN,
}

pub(crate) struct User {
    id: Uuid,
    email: String,
    username: String,
    first_name: String,
    last_name: String,
    age: usize,
    gender: Gender,
    is_banned: bool,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        username: String,
        first_name: String,
        age: usize,
        last_name: String,
        gender: Gender,
    ) -> Self {
        Self {
            id,
            email,
            username,
            age,
            first_name,
            last_name,
            gender,
            is_banned: false,
        }
    }
}
