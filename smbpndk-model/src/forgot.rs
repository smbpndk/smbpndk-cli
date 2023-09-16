use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Args {
    pub user: Email,
}

#[derive(Debug, Serialize)]
pub struct Email {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct Param {
    pub user: UserUpdatePassword,
}

#[derive(Debug, Serialize)]
pub struct UserUpdatePassword {
    pub reset_password_token: String,
    pub password: String,
    pub password_confirmation: String,
}
