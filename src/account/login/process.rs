use std::{
    fs::{self, create_dir_all, OpenOptions},
    io::Write,
};

use anyhow::{anyhow, Result};
use console::style;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use spinners::Spinner;

use crate::{
    account::model::{Data, Status, User},
    constants::BASE_URL,
    debug,
    util::CommandResult,
};

pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
struct LoginParams {
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResult {
    status: Status,
    data: Data,
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

pub async fn process_logout() -> Result<CommandResult> {
    let spinner = Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("⏳ Logging you out...").green().bold().to_string(),
    );
    match home::home_dir() {
        Some(path) => {
            debug!(path.to_str().unwrap());
            fs::remove_file([path.to_str().unwrap(), "/.smb/token"].join(""))?;

            Ok(CommandResult {
                spinner,
                symbol: "✅".to_owned(),
                msg: "You are logged out!".to_owned(),
            })
        }
        None => {
            let error = anyhow!("Failed to get home directory. Are you logged in?");
            return Err(error);
        }
    }
}
