use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub(crate) email: String,
    pub(crate) password: String,
}
