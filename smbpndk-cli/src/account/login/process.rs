use crate::account::{
    lib::{authorize_github, smb_base_url_builder, ErrorCode, GithubInfo, SmbAuthorization},
    model::{Data, Status, User},
    signup::{
        do_signup, GithubEmail, Provider, SignupGithubParams, SignupMethod, SignupUserGithub,
    },
};
use anyhow::{anyhow, Result};
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};
use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use smbpndk_model::CommandResult;
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
    user: UserParam,
}

#[derive(Debug, Serialize)]
struct UserParam {
    email: String,
    password: String,
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
        Ok(result) => process_authorization(result).await,
        Err(err) => {
            let error = anyhow!("Failed to authorize your GitHub account. {}", err);
            Err(error)
        }
    }
}

async fn process_authorization(auth: SmbAuthorization) -> Result<CommandResult> {
    // What to do if not logged in with GitHub?
    // Check error_code first
    if let Some(error_code) = auth.error_code {
        debug!("{}", error_code);
        match error_code {
            ErrorCode::EmailNotFound => return create_new_account(auth.user_email, auth.user_info).await,
            ErrorCode::EmailUnverified => return send_email_verification(auth.user).await,
        }
    }

    // Logged in with GitHub
    if let Some(user) = auth.user {
        print!("You are logged in with GitHub as {}.", user);
        let spinner = Spinner::new(
            spinners::Spinners::SimpleDotsScrolling,
            style("Logging you in...").green().bold().to_string(),
        );
        // We're logged in with GitHub, but not with SMB.
        return Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: "You are logged in!".to_owned(),
        });
    }

    let error: anyhow::Error = anyhow!("Failed to login with GitHub.");
    Err(error)
}

async fn create_new_account(
    user_email: Option<GithubEmail>,
    user_info: Option<GithubInfo>,
) -> Result<CommandResult> {
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
            msg: "Please accept to link your GitHub account.".to_owned(),
        });
    }

    if let (Some(email), Some(info)) = (user_email, user_info) {
        let params = SignupGithubParams {
            user: SignupUserGithub {
                email: email.email,
                authorizations_attributes: vec![Provider {
                    uid: info.id.to_string(),
                    provider: 0,
                }],
            },
        };

        return do_signup(&params).await;
    }

    Err(anyhow!("Shouldn't be here."))
}

async fn send_email_verification(user: Option<User>) -> Result<CommandResult> {
    // Return early if user is null
    if let Some(user) = user {

        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to send a new verification email?")
            .interact()
            .unwrap();

        // Send verification email if user confirms
        if !confirm {
            let spinner = Spinner::new(
                spinners::Spinners::SimpleDotsScrolling,
                style("Cancel operation.").green().bold().to_string(),
            );
            return Ok(CommandResult {
                spinner,
                symbol: "✅".to_owned(),
                msg: "Doing nothing.".to_owned(),
            });
        }
        resend_email_verification(user).await
    } else {
        let error = anyhow!("Failed to get user.");
        Err(error)
    }
}

async fn resend_email_verification(user: User) -> Result<CommandResult> {
    let spinner = Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("Sending verification email...").green().bold().to_string(),
    );

    let response = Client::new()
        .post(build_smb_resend_email_verification_url())
        .body(format!("id={}", user.id))
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            Ok(CommandResult {
                spinner,
                symbol: "✅".to_owned(),
                msg: "Verification email sent!".to_owned(),
            })
        }
        _ => {
            let error = anyhow!("Failed to send verification email.");
            Err(error)
        }
    }
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
        user: UserParam {
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

fn build_smb_resend_email_verification_url() -> String {
    let mut url_builder = smb_base_url_builder();
    url_builder.add_route("v1/resend_confirmation");
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
