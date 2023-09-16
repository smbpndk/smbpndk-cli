use crate::account::{Data, Status};
use serde::{Deserialize, Serialize};

pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginParams {
    pub user: UserParam,
}

#[derive(Debug, Serialize)]
pub struct UserParam {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub status: Status,
    pub data: Data,
}
