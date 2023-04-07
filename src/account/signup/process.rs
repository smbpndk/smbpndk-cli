use std::{
    fmt::{Display, Formatter},
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};

use anyhow::{anyhow, Result};
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use regex::Regex;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use spinners::Spinner;
use url_builder::URLBuilder;

use crate::{
    account::model::{Data, Status, User},
    constants::BASE_URL,
    debug,
    util::CommandResult,
};
pub struct SignupArgs {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct SignupParams {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignupResult {
    status: Status,
    data: Option<Data>,
}

pub async fn signup_with_email(email: Option<String>) -> Result<CommandResult> {
    let email = if let Some(email) = email {
        email
    } else {
        Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Username")
                .validate_with(|input: &String| -> Result<(), &str> {
                    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

                    if email_regex.is_match(input) {
                        Ok(())
                    } else {
                        Err("Username must be an email address")
                    }
                })
                .interact()
                .unwrap()
    };

    let password = Password::with_theme(&ColorfulTheme::default())
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

    let spinner = Spinner::new(
        spinners::Spinners::BouncingBall,
        style("Signing up...").green().bold().to_string(),
    );

    match process_signup(SignupArgs {
        username: email,
        password,
    })
    .await
    {
        Ok(_) => Ok(CommandResult {
            spinner,
            symbol: style("✔".to_string()).for_stderr().green().to_string(),
            msg: "You are signed up! Check your email to confirm your account.".to_owned(),
        }),
        Err(e) => Ok(CommandResult {
            spinner,
            symbol: style("✘".to_string()).for_stderr().red().to_string(),
            msg: format!("{e}"),
        }),
    }
}

pub async fn signup_with_github() -> Result<CommandResult> {
    // Spin up a simple localhost server to listen for the GitHub OAuth callback
    // setup_oauth_callback_server();
    // Open the GitHub OAuth URL in the user's browser
    let mut spinner = Spinner::new(
        spinners::Spinners::BouncingBall,
        style("Getting your GitHub information...")
            .green()
            .bold()
            .to_string(),
    );

    let rx = match open::that(build_github_oauth_url()) {
        Ok(_) => {
            let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
            debug!(
                "Setting up OAuth callback server... (tx: {:#?}, rx: {:#?})",
                &tx, &rx
            );
            tokio::spawn(async move {
                setup_oauth_callback_server(tx);
            });
            rx
        }
        Err(_) => {
            let error = anyhow!("Failed to open a browser.");
            return Err(error);
        }
    };

    spinner.stop_and_persist("⌛", "Waiting for the authorization.".into());

    debug!("Waiting for code from channel...");

    match rx.recv() {
        Ok(code) => {
            debug!("Got code from channel: {:#?}", &code);
            process_signup_github(code).await
        }
        Err(e) => {
            let error = anyhow!("Failed to get code from channel: {e}");
            Err(error)
        }
    }
}

fn setup_oauth_callback_server(tx: Sender<String>) {
    let listener = TcpListener::bind("127.0.0.1:8808").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, tx.clone());
    }
}

fn handle_connection(mut stream: TcpStream, tx: Sender<String>) {
    let buf_reader = BufReader::new(&stream);
    let request_line = &buf_reader.lines().next().unwrap().unwrap();

    debug!("Request: {:#?}", request_line);

    let code_regex = Regex::new(r"code=([^&]*)").unwrap();

    let (status_line, filename) = match code_regex.captures(request_line) {
        Some(group) => {
            let code = group.get(1).unwrap().as_str();
            debug!("Code: {:#?}", code);
            debug!("Sending code to channel...");
            debug!("Channel: {:#?}", &tx);
            match tx.send(code.to_string()) {
                Ok(_) => {
                    debug!("Code sent to channel.");
                }
                Err(e) => {
                    debug!("Failed to send code to channel: {e}", e);
                }
            }
            ("HTTP/1.1 200 OK", "./src/account/hello.html")
        }
        None => {
            debug!("Code not found.");
            ("HTTP/1.1 404 NOT FOUND", "./src/account/404.html")
        }
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct GithubToken {
    access_token: String,
    scope: String,
    token_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct GithubUser {
    email: Option<String>,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
    visibility: Option<String>,
}

impl Display for GithubEmail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

// Get access token
async fn process_signup_github(code: String) -> Result<CommandResult> {
    let response = Client::new()
        .post(build_github_access_token_url(code))
        .header("Accept", "application/json")
        .send()
        .await?;
    let mut spinner = Spinner::new(
        spinners::Spinners::BouncingBall,
        style("Requesting GitHub token...")
            .green()
            .bold()
            .to_string(),
    );
    match response.status() {
        StatusCode::OK => {
            spinner.stop_and_persist("✔", "Finished requesting GitHub token!".into());
            debug!("Response: {:#?}", &response);
            let result: GithubToken = response.json().await?;
            debug!("Result: {:#?}", &result);
            get_github_data(result.access_token).await
        }
        _ => {
            let error = anyhow!("Error while requesting GitHub token.");
            Err(error)
        }
    }
}

// Request user data
async fn get_github_data(token: String) -> Result<CommandResult> {
    let mut spinner = Spinner::new(
        spinners::Spinners::BouncingBall,
        style("Requesting GitHub data...")
            .green()
            .bold()
            .to_string(),
    );
    let response = Client::new()
        .get(build_github_access_data_url())
        .header("Accept", "application/json")
        .header("User-Agent", "Rust")
        .bearer_auth(token)
        .send()
        .await?;

    debug!("Response: {:#?}", &response);

    match response.status() {
        StatusCode::OK => {
            spinner.stop_and_persist("✔", "Finished requesting GitHub token!".into());
            debug!("Response: {:#?}", &response);
            let mut emails: Vec<GithubEmail> = response.json().await?;
            debug!(&emails);
            emails.retain(|e| !e.email.contains("@users.noreply.github.com"));

            let email = select_github_emails(emails)?;
            signup_with_email(Some(email.email)).await
        }
        status_code => {
            let error = anyhow!("Error while requesting GitHub data: {status_code}");
            Err(error)
        }
    }
}

fn select_github_emails(github_emails: Vec<GithubEmail>) -> Result<GithubEmail> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your email")
        .items(&github_emails)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .map(|i| &github_emails[i.unwrap()])
        .unwrap()
        .to_owned();
    Ok(selection)
}

fn github_url_builder() -> URLBuilder {
    let client_id =
        dotenv::var("GH_OAUTH_CLIENT_ID").unwrap_or("Please set GH_OAUTH_CLIENT_ID".to_owned());
    let redirect_uri = dotenv::var("GH_OAUTH_REDIRECT_URI")
        .unwrap_or("Please set GH_OAUTH_REDIRECT_URI".to_owned());
    let mut url_builder = URLBuilder::new();
    url_builder
        .set_protocol("https")
        .set_host("github.com")
        .add_param("client_id", &client_id)
        .add_param("redirect_uri", &redirect_uri);
    url_builder
}

fn build_github_oauth_url() -> String {
    let mut url_builder = github_url_builder();
    url_builder
        .add_route("login/oauth/authorize")
        .add_param("scope", "user")
        .add_param("state", "smbpndk");
    url_builder.build()
}

fn build_github_access_token_url(code: String) -> String {
    let client_secret = dotenv::var("GH_OAUTH_CLIENT_SECRET").unwrap_or("development".to_owned());
    let mut url_builder = github_url_builder();
    url_builder
        .add_route("login/oauth/access_token")
        .add_param("client_secret", &client_secret)
        .add_param("code", &code);
    url_builder.build()
}

fn build_github_access_data_url() -> String {
    let mut url_builder = URLBuilder::new();
    url_builder
        .set_protocol("https")
        .set_host("api.github.com")
        .add_route("user/emails");
    url_builder.build()
}

async fn process_signup(args: SignupArgs) -> Result<()> {
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
        StatusCode::OK => {}
        StatusCode::UNPROCESSABLE_ENTITY => {
            let result: SignupResult = response.json().await?;
            let error = anyhow!("Failed to signup: {}", result.status.message);
            return Err(error);
        }
        _ => {
            let error = anyhow!("Failed to signup: {}", response.status());
            return Err(error);
        }
    }

    Ok(())
}
