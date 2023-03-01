use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Serialize;

use crate::{constants::BASE_URL, login::User};
pub struct SignupArgs {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct SignupParams {
    pub user: User,
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
        reqwest::StatusCode::OK => {
            let headers = response.headers();
            let token = headers.get("Authorization").unwrap().to_str().unwrap();
            println!("token: {token}");
        }
        _ => {
            let error = anyhow!("Failed to signup: {}", response.status());
            return Err(error);
        }
    }

    Ok(())
}
