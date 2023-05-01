use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct User {
    pub(crate) email: String,
    pub(crate) password: String,
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
