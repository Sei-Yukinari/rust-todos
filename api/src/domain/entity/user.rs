use validator::Validate;
use crate::infrastructure::user_repository::UserRow;
use crate::use_case::dto::user::UserDto;

// use crate::{domain::error::DomainError, impl_string_value_object};

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct UserId {
    value: i64,
}

// impl_string_value_object!(UserId);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub name: String,
}

impl User {
    pub fn new(row: UserRow) -> User {
        User {
            id: row.id,
            name: row.name,
        }
    }
}