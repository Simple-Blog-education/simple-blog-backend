use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    Unknown,
    User,
    Admin
}

impl UserRole {
    pub fn value(&self) -> usize {
        match *self {
            UserRole::Unknown => 0,
            UserRole::User => 1,
            UserRole::Admin => 2
        }
    }
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        let slice = value.as_str();
        match slice {
            "User" => Self::User,
            "Admin" => Self::Admin,
            _ => Self::Unknown
        }
    }
}

impl From<Option<String>> for UserRole {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => Self::from(value),
            None => Self::Unknown
        }
    }
}
