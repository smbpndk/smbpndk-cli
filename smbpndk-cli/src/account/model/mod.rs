use std::fmt::{Formatter, Display};

use serde::{Deserialize, Serialize};

// SMBPNDK Users.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) email: String,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: id: {}, email: {}", self.id, self.email)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    code: Option<i32>,
    pub(crate) message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    id: i32,
    email: String,
    created_at: String,
}
