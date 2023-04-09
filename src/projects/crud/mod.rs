use crate::{constants::BASE_URL, debug};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Serialize, Debug)]
pub struct ProjectCreate {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub(crate) current_project: Option<Project>,
}

pub async fn get_all() -> Result<Vec<Project>> {
    // Get current token
    let token = get_token().await.unwrap();

    let response = Client::new()
        .get([BASE_URL, "v1/projects"].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let projects: Vec<Project> = response.json().await?;
            Ok(projects)
        }
        _ => {
            debug!("Failed to get all projects.", response.status());
            Err(anyhow!("Failed to fetch projects."))
        }
    }
}

pub async fn create_project(project: ProjectCreate) -> Result<Project> {
    // Get current token
    let token = get_token().await?;

    let response = Client::new()
        .post([BASE_URL, "v1/projects"].join(""))
        .json(&project)
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => {
            let project: Project = response.json().await?;
            println!("Project created: {project:#?}");
            Ok(project)
        }
        _ => {
            debug!("Failed to create a project.", response.status());
            Err(anyhow!("Failed to create a project."))
        }
    }
}

pub async fn get_project(id: String) -> Result<Project> {
    // Get current token
    let token = get_token().await.unwrap();

    let response = Client::new()
        .get([BASE_URL, "v1/projects/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let project: Project = response.json().await?;
            println!("Project requested: {project:#?}");
            Ok(project)
        }
        _ => {
            debug!("Failed to request a project.", response.status());
            Err(anyhow!("Failed to request a project."))
        }
    }
}

pub async fn delete_project(id: String) -> Result<()> {
    // Get current token
    let token = get_token().await.unwrap();

    let response = Client::new()
        .delete([BASE_URL, "v1/projects/", &id].join(""))
        .header("Authorization", token)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("Project deleted.");
            Ok(())
        }
        _ => {
            debug!("Failed to request a project.", response.status());
            Err(anyhow!("Failed to delete a project."))
        }
    }
}

async fn get_token() -> Result<String> {
    if let Some(mut path) = dirs::home_dir() {
        path.push(".smb/token");
        std::fs::read_to_string(path).map_err(|e| {
            debug!("Error while reading token: {}", &e);
            anyhow!("Are you logged in?")
        })
    } else {
        Err(anyhow!("Failed to get home directory."))
    }
}
