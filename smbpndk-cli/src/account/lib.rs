use anyhow::{anyhow, Result};
use console::style;
use log::debug;
use regex::Regex;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use smbpndk_model::CommandResult;
use spinners::Spinner;
use std::{
    env,
    fmt::{Display, Formatter},
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};
use url_builder::URLBuilder;

// This is smb authorization model.
#[derive(Debug, Serialize, Deserialize)]
pub struct SmbAuthorization {
    access_token: String,
    scope: String,
    token_type: String,
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
            process_connect_github(code).await
        }
        Err(e) => {
            let error = anyhow!("Failed to get code from channel: {e}");
            Err(error)
        }
    }
}

fn setup_oauth_callback_server(tx: Sender<String>) {
    let port = env::var("GH_OAUTH_REDIRECT_PORT").expect("Please set GH_OAUTH_REDIRECT_PORT");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
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

// Get access token
pub async fn process_connect_github(code: String) -> Result<SmbAuthorization> {
    let response = Client::new()
        .post(build_authorize_smb_url(code))
        .header("Accept", "application/json")
        .send()
        .await?;
    let mut spinner = Spinner::new(
        spinners::Spinners::BouncingBall,
        style("🚀 Requesting GitHub token...")
            .green()
            .bold()
            .to_string(),
    );
    match response.status() {
        StatusCode::OK => {
            spinner.stop_and_persist("✅", "Finished requesting GitHub token!".into());
            debug!("Response: {:#?}", &response);
            let result: SmbAuthorization = response.json().await?;
            debug!("Result: {:#?}", &result);
            Ok(result)
        }
        _ => {
            let error = anyhow!("Error while requesting GitHub token.");
            Err(error)
        }
    }
}

fn build_authorize_smb_url(code: String) -> String {
    let mut url_builder = smb_base_url_builder();
    url_builder
        .add_route("v1/authorize")
        .add_param("gh_code", &code);
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

fn smb_base_url_builder() -> URLBuilder {
    let client_id = env::var("SMB_CLIENT_ID").expect("Please set SMB_CLIENT_ID");
    let client_secret = env::var("SMB_CLIENT_SECRET").expect("Please set SMB_CLIENT_SECRET");
    let mut url_builder = URLBuilder::new();
    url_builder
        .set_protocol("https")
        .set_host("api.smbpndk.com")
        .add_param("client_id", &client_id)
        .add_param("client_secret", &client_secret);
    url_builder
}

fn github_base_url_builder() -> URLBuilder {
    let client_id = env::var("GH_OAUTH_CLIENT_ID").expect("Please set GH_OAUTH_CLIENT_ID");
    let redirect_host =
        env::var("GH_OAUTH_REDIRECT_HOST").expect("Please set GH_OAUTH_REDIRECT_HOST");
    let redirect_port =
        env::var("GH_OAUTH_REDIRECT_PORT").expect("Please set GH_OAUTH_REDIRECT_PORT");
    let redirect_url = format!("{}:{}", &redirect_host, &redirect_port);

    let mut url_builder = URLBuilder::new();
    url_builder
        .set_protocol("https")
        .set_host("github.com")
        .add_param("client_id", &client_id)
        .add_param("redirect_uri", &redirect_url);
    url_builder
}
