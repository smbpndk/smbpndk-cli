use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
};

use anyhow::{anyhow, Result};
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
pub struct User {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResult {
    status: Status,
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    code: i32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    id: i32,
    email: String,
    created_at: String,
}

macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                dbg!($($e),+)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($e),+)
            }
        }
    };
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
            let headers = response.headers();
            match headers.get("Authorization") {
                Some(token) => {
                    debug!(token.to_str()?);
                    match home::home_dir() {
                        Some(path) => {
                            debug!(path.to_str().unwrap());
                            create_dir_all(path.join(".smb"))?;
                            let mut file = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open([path.to_str().unwrap(), "/.smb/token"].join(""))?;
                            file.write_all(token.to_str()?.as_bytes())?;
                        }
                        None => {
                            let error = anyhow!("Failed to get home directory.");
                            return Err(error);
                        }
                    }
                }
                None => {
                    let error = anyhow!("Failed to get token. Probably a backend issue.");
                    return Err(error);
                }
            }
        }
        _ => {
            let error = anyhow!("Connection error. Check your username and password.");
            return Err(error);
        }
    }

    Ok(())
}
