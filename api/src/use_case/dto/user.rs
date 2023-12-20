use crate::domain::entity::user::User as DomainUser;

pub struct UserDto {
    pub id: i64,
    pub name: String,
}

impl UserDto {
    pub fn new(user: DomainUser) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}

impl From<DomainUser> for UserDto {
    fn from(user: DomainUser) -> Self {
        UserDto::new(user)
    }
}
