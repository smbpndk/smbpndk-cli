use crate::account::model::{Data, Status, User};
use anyhow::{anyhow, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use smbpndk_model::CommandResult;
use smbpndk_networking::constants::BASE_URL;
use smbpndk_utils::email_validation;
use spinners::Spinner;
use std::{
    fs::{self, create_dir_all, OpenOptions},
    io::Write,
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

pub async fn process_login() -> Result<CommandResult> {
    println!("Provide your login credentials.");
    let username = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Username")
        .validate_with(|email: &String| email_validation(email))
        .interact()
        .unwrap();
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
        .unwrap();

    let spinner = Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("Logging in...").green().bold().to_string(),
    );

    match do_process_login(LoginArgs { username, password }).await {
        Ok(_) => Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: "You are logged in!".to_owned(),
        }),
        Err(e) => Ok(CommandResult {
            spinner,
            symbol: "😩".to_owned(),
            msg: format!("Failed to login: {e}"),
        }),
    }
}

async fn do_process_login(args: LoginArgs) -> Result<()> {
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
                    debug!("{}", token.to_str()?);
                    match home::home_dir() {
                        Some(path) => {
                            debug!("{}", path.to_str().unwrap());
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
        style("Logging you out...").green().bold().to_string(),
    );
    match home::home_dir() {
        Some(path) => {
            debug!("{}", path.to_str().unwrap());
            fs::remove_file([path.to_str().unwrap(), "/.smb/token"].join(""))?;

            Ok(CommandResult {
                spinner,
                symbol: "✅".to_owned(),
                msg: "You are logged out!".to_owned(),
            })
        }
        None => {
            let error = anyhow!("Failed to get home directory. Are you logged in?");
            Err(error)
        }
    }
}
