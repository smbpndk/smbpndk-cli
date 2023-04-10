use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use spinners::Spinner;

use crate::{
    constants::BASE_URL,
    util::{email_validation, CommandResult},
};

#[derive(Debug, Serialize)]
struct Args {
    user: Email,
}

#[derive(Debug, Serialize)]
struct Email {
    email: String,
}

pub async fn process_forgot() -> Result<CommandResult> {
    println!("Provide your login credentials.");
    let email = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Email")
        .validate_with(|email: &String| email_validation(email))
        .interact()
        .unwrap();
    let mut spinner = Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("Checking email...").green().bold().to_string(),
    );

    let params = Args {
        user: Email { email },
    };

    let response = Client::new()
        .post([BASE_URL, "/v1/users/password"].join(""))
        .json(&params)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            spinner.stop_and_persist(
                "✅",
                "Check your email and input your code here.".to_owned(),
            );
            input_code().await
        }
        _ => Ok(CommandResult {
            spinner,
            symbol: "😩".to_owned(),
            msg: "Something wrong when trying to reset email.".to_owned(),
        }),
    }
}

async fn input_code() -> Result<CommandResult> {
    let security_code = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Code")
        .interact()
        .unwrap();

    Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("Checking your code...").green().bold().to_string(),
    )
    .stop_and_persist(
        &"✅".to_owned(),
        "Great. Now input your new password.".to_owned(),
    );

    let new_password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() >= 6 {
                Ok(())
            } else {
                Err("Password must be at least 6 characters")
            }
        })
        .with_confirmation("Confirm password", "Passwords do not match")
        .interact()
        .unwrap();
    let password_confirmation = String::from(&new_password);

    #[derive(Debug, Serialize)]
    struct Param {
        user: User,
    }

    #[derive(Debug, Serialize)]
    struct User {
        reset_password_token: String,
        password: String,
        password_confirmation: String,
    }

    let params = Param {
        user: User {
            reset_password_token: security_code,
            password: new_password,
            password_confirmation: password_confirmation,
        },
    };

    let mut spinner = Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("Updating your password...")
            .green()
            .bold()
            .to_string(),
    );

    let response = Client::new()
        .put([BASE_URL, "/v1/users/password"].join(""))
        .json(&params)
        .send()
        .await?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Response {
        status: i32,
        email: Option<String>,
    }

    match response.status() {
        StatusCode::OK => Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: "Your password has been updated. Login with your new password.".to_owned(),
        }),
        StatusCode::NOT_FOUND => Ok(CommandResult {
            spinner,
            symbol: "😩".to_owned(),
            msg: "URL not found.".to_owned(),
        }),
        _ => Ok(CommandResult {
            spinner,
            symbol: "😩".to_owned(),
            msg: "Something wrong when trying to reset email.".to_owned(),
        }),
    }
}
