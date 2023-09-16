pub mod account;
pub mod forgot;
pub mod login;
pub mod signup;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use spinners::Spinner;

mod ar_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%#z";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.naive_utc());
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub struct CommandResult {
    pub spinner: Spinner,
    pub symbol: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthApp {
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    #[serde(with = "ar_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ar_date_format")]
    pub updated_at: DateTime<Utc>,
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
    pub description: String,
    #[serde(with = "ar_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ar_date_format")]
    pub updated_at: DateTime<Utc>,
}
#[derive(Serialize, Debug)]
pub struct ProjectCreate {
    pub name: String,
    pub description: String,
}
