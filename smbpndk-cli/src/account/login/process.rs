use crate::account::{
    lib::{authorize_github, SmbAuthorization, ErrorCode, GithubInfo, smb_base_url_builder},
    model::{Data, Status, User},
    signup::{SignupMethod, GithubEmail, do_signup, SignupGithubParams, SignupUserGithub, Provider},
};
use anyhow::{anyhow, Result};
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Password, Select, Confirm};
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
    let signup_methods = vec![SignupMethod::Email, SignupMethod::GitHub];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&signup_methods)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .map(|i| signup_methods[i.unwrap()])
        .unwrap();

    match selection {
        SignupMethod::Email => login_with_email().await,
        SignupMethod::GitHub => login_with_github().await,
    }
}

async fn login_with_github() -> Result<CommandResult> {
    match authorize_github().await {
        Ok(result) => {
            process_authorization(result).await
        }
        Err(err) => {
            let error = anyhow!("Failed to authorize your GitHub account. {}", err);
            Err(error)
        }
    }
}

async fn process_authorization(auth: SmbAuthorization) -> Result<CommandResult>  {
    // Logged in with GitHub
    if let Some(user) = auth.user {
        let spinner = Spinner::new(
            spinners::Spinners::SimpleDotsScrolling,
            style("Logging you in...").green().bold().to_string(),
        );
        return Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: "You are logged in!".to_owned(),
        })
    }

    // What to do if not logged in with GitHub?
    if let Some(error_code) = auth.error_code {
        debug!("{}", error_code);
        match error_code {
            ErrorCode::EmailNotFound => {
                return create_new_account(auth.user_email, auth.user_info).await
            },
            ErrorCode::EmailUnverified => {
                return send_email_verification(auth.user_email).await
            }
        }
    }

    let error = anyhow!("Failed to login with GitHub.");
    Err(error)

}

async fn create_new_account(user_email: Option<GithubEmail>, user_info: Option<GithubInfo>) -> Result<CommandResult> {
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to create a new account?")
        .interact()
        .unwrap();

    // Create account if user confirms
    if !confirm {
        let spinner = Spinner::new(
            spinners::Spinners::SimpleDotsScrolling,
            style("Logging you in...").green().bold().to_string(),
        );
        return Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: "You are logged in!".to_owned(),
        })
    }

    if let (Some(email), Some(info)) = (user_email, user_info) {
        let params = SignupGithubParams {
            user: SignupUserGithub {
                email: email.email,
                authorizations_attributes: vec![Provider { uid: info.id.to_string(), provider: 0 }],
            } 
        };

        return do_signup(&params).await
    }

    Err(anyhow!("Shouldn't be here."))
}

async fn send_email_verification(user_email: Option<GithubEmail>) -> Result<CommandResult> {
    Err(anyhow!("Failed to send email verification."))
}

async fn login_with_email() -> Result<CommandResult> {
    println!("Provide your login credentials.");
    let username = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Email")
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
        .post(build_smb_login_url())
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

fn build_smb_login_url() -> String {
    let mut url_builder = smb_base_url_builder();
    url_builder.add_route("v1/users/sign_in");
    url_builder.build()
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
