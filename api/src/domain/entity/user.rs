use validator::Validate;

use crate::gateway::user_repository::UserRow;

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct UserId {
    value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub name: String,
}

impl User {
    pub fn new(row: UserRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn user_new_creates_user_from_row() {
        let row = UserRow {
            id: 1,
            name: "Test User".to_string(),
        };

        let user = User::new(row.clone());

        assert_eq!(user.id, row.id);
        assert_eq!(user.name, row.name);
    }

    #[tokio::test]
    async fn user_new_handles_empty_name() {
        let row = UserRow {
            id: 1,
            name: "".to_string(),
        };

        let user = User::new(row.clone());

        assert_eq!(user.id, row.id);
        assert_eq!(user.name, row.name);
    }
}