use async_graphql::{ID, SimpleObject};
use crate::use_case::dto::user::UserDto;

#[derive(SimpleObject)]
pub struct User {
    id: ID,
    name: String,
}

impl User {
    pub fn new(user: UserDto) -> Self {
        Self {
            id: user.id.to_string().into(),
            name: user.name.to_string().into(),
        }
    }
}