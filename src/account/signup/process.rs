use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    account::model::{Data, Status, User},
    constants::BASE_URL,
};
pub struct SignupArgs {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct SignupParams {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignupResult {
    status: Status,
    data: Option<Data>,
}

pub async fn process_signup(args: SignupArgs) -> Result<()> {
    let signup_params = SignupParams {
        user: User {
            email: args.username,
            password: args.password,
        },
    };

    let response = Client::new()
        .post([BASE_URL, "/v1/users"].join(""))
        .json(&signup_params)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {}
        reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
            let result: SignupResult = response.json().await?;
            let error = anyhow!("Failed to signup: {}", result.status.message);
            return Err(error);
        }
        _ => {
            let error = anyhow!("Failed to signup: {}", response.status());
            return Err(error);
        }
    }

    Ok(())
}
