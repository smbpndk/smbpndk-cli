use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use spinners::Spinner;

use crate::{constants::BASE_URL, util::CommandResult};

#[derive(Debug, Serialize)]
struct Args {
    user: Email,
}

#[derive(Debug, Serialize)]
struct Email {
    email: String,
}

pub async fn process() -> Result<CommandResult> {
    println!("Provide your login credentials.");
    let email = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Email")
        .interact()
        .unwrap();
    let spinner = Spinner::new(
        spinners::Spinners::SimpleDotsScrolling,
        style("⏳ Logging in...").green().bold().to_string(),
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
        StatusCode::OK => Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: "Check your email and follow the instruction to reset your password.".to_owned(),
        }),
        _ => Ok(CommandResult {
            spinner,
            symbol: "✅".to_owned(),
            msg: "You are logged in!".to_owned(),
        }),
    }
}
