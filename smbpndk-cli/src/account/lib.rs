use super::{model::User, signup::GithubEmail};
use anyhow::{anyhow, Result};
use console::style;
use log::debug;
use regex::Regex;
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use smbpndk_networking::{
    constants::{GH_OAUTH_CLIENT_ID, GH_OAUTH_REDIRECT_HOST, GH_OAUTH_REDIRECT_PORT},
    smb_base_url_builder,
};
use spinners::Spinner;
use std::{
    fmt::{Display, Formatter},
    fs::{self, create_dir_all, OpenOptions},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};
use url_builder::URLBuilder;

// This is smb authorization model.
#[derive(Debug, Serialize, Deserialize)]
pub struct SmbAuthorization {
    pub message: String,
    pub user: Option<User>,
    pub user_email: Option<GithubEmail>,
    pub user_info: Option<GithubInfo>,
    pub error_code: Option<ErrorCode>,
}

#[derive(Debug, serde_repr::Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub enum ErrorCode {
    EmailNotFound = 1000,
    EmailUnverified = 1001,
    PasswordNotSet = 1003,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::EmailNotFound => write!(f, "Email not found."),
            ErrorCode::EmailUnverified => write!(f, "Email not verified."),
            ErrorCode::PasswordNotSet => write!(f, "Password not set."),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubInfo {
    pub id: i64,
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub html_url: String,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn authorize_github() -> Result<SmbAuthorization> {
    // Spin up a simple localhost server to listen for the GitHub OAuth callback
    // setup_oauth_callback_server();
    // Open the GitHub OAuth URL in the user's browser
    let mut spinner = Spinner::new(
        spinners::Spinners::BouncingBall,
        style("🚀 Getting your GitHub information...")
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
            //Err(anyhow!("Failed to get code from channel."))
            process_connect_github(code).await
        }
        Err(e) => {
            let error = anyhow!("Failed to get code from channel: {e}");
            Err(error)
        }
    }
}

fn setup_oauth_callback_server(tx: Sender<String>) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", GH_OAUTH_REDIRECT_PORT)).unwrap();
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
                    debug!("Failed to send code to channel: {e}");
                }
            }
            ("HTTP/1.1 200 OK", "./smbpndk-cli/src/account/hello.html")
        }
        None => {
            debug!("Code not found.");
            (
                "HTTP/1.1 404 NOT FOUND",
                "./smbpndk-cli/src/account/404.html",
            )
        }
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Get access token
pub async fn process_connect_github(code: String) -> Result<SmbAuthorization> {
    let response = Client::new()
        .post(build_authorize_smb_url())
        .body(format!("gh_code={}", code))
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    let mut spinner = Spinner::new(
        spinners::Spinners::BouncingBall,
        style("🚀 Authorizing your account...")
            .green()
            .bold()
            .to_string(),
    );
    // println!("Response: {:#?}", &response);
    match response.status() {
        StatusCode::OK => {
            // Account authorized and token received
            spinner.stop_and_persist("✅", "You are logged in with your GitHub account!".into());
            save_token(&response).await?;
            let result = response.json().await?;
            // println!("Result: {:#?}", &result);
            Ok(result)
        }
        StatusCode::NOT_FOUND => {
            // Account not found and we show signup option
            spinner.stop_and_persist("🥲", "Account not found. Please signup!".into());
            let result = response.json().await?;
            // println!("Result: {:#?}", &result);
            Ok(result)
        }
        StatusCode::UNPROCESSABLE_ENTITY => {
            // Account found but email not verified
            spinner.stop_and_persist("🥹", "Unverified email!".into());
            let result = response.json().await?;
            // println!("Result: {:#?}", &result);
            Ok(result)
        }
        _ => {
            // Other errors
            let error = anyhow!("Error while authorizing with GitHub.");
            Err(error)
        }
    }
}

fn build_authorize_smb_url() -> String {
    let mut url_builder = smb_base_url_builder();
    url_builder.add_route("v1/authorize");
    url_builder.build()
}

fn build_github_oauth_url() -> String {
    let mut url_builder = github_base_url_builder();
    url_builder
        .add_route("login/oauth/authorize")
        .add_param("scope", "user")
        .add_param("state", "smbpndk");
    url_builder.build()
}

fn github_base_url_builder() -> URLBuilder {
    let redirect_url = format!("{}:{}", GH_OAUTH_REDIRECT_HOST, GH_OAUTH_REDIRECT_PORT);

    let mut url_builder = URLBuilder::new();
    url_builder
        .set_protocol("https")
        .set_host("github.com")
        .add_param("client_id", GH_OAUTH_CLIENT_ID)
        .add_param("redirect_uri", &redirect_url);
    url_builder
}

pub async fn save_token(response: &Response) -> Result<()> {
    let headers = response.headers();
    // println!("Headers: {:#?}", &headers);
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
                    Ok(())
                }
                None => Err(anyhow!("Failed to get home directory.")),
            }
        }
        None => Err(anyhow!("Failed to get token. Probably a backend issue.")),
    }
}
