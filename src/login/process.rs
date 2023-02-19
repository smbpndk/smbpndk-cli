use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::constants::BASE_URL;

pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
struct LoginParams {
    user: User,
}

#[derive(Debug, Serialize)]
struct User {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResult {
    status: i32,
    error: String,
}

pub async fn process_login(args: LoginArgs) -> Result<()> {
    let login_params = LoginParams {
        user: User {
            email: args.username,
            password: args.password,
        },
    };

    let response = Client::new()
        .post([BASE_URL, "/v1/users/sign_in"].join(""))
        .json(&login_params)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let login_result: LoginResult = response.json().await?;
            println!("Login result: {:?}", login_result);
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("UNAUTHORIZED");
        }
        _ => {
            println!("Login failed: {:?}", response);
        }
    }

    Ok(())
}
