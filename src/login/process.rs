use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

    let response: LoginResult = Client::new()
        .post("http://localhost:3000/v1/users/sign_in")
        .json(&login_params)
        .send()
        .await?
        .json()
        .await?;

    println!("Response: {:?}", response);
    Ok(())
}
