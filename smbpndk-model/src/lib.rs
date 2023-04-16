use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthApp {
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthAppCreate {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub current_project: Option<Project>,
    pub current_auth_app: Option<AuthApp>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_auth_app() {
        let data = r#"{

        }"#;
        //let auth_app = serde_json::from_value(data.into());
    }
}
