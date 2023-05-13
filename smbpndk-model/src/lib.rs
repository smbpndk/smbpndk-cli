use std::fmt::Display;

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
pub struct Oten {
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    #[serde(with = "ar_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ar_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fun {
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    #[serde(with = "ar_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ar_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pkt {
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    #[serde(with = "ar_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ar_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rdb {
    pub id: String,
    pub secret: Option<String>,
    pub name: String,
    #[serde(with = "ar_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ar_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppCreate {
    pub name: String,
    pub description: String,
}

pub mod create_params {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug, Serialize)]
    pub struct Oten {
        pub oten_app: AppCreate,
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct AppCreate {
        pub name: String,
        pub project_id: i32,
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub current_project: Option<Project>,
    pub current_oten_app: Option<Oten>,
    pub current_fun_app: Option<Fun>,
    pub current_pkt_app: Option<Pkt>,
    pub current_rdb_app: Option<Rdb>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
    #[serde(with = "ar_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ar_date_format")]
    pub updated_at: DateTime<Utc>,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id.to_string();
        let name = self.name.to_string();
        let created_at = self.created_at.date_naive();
        let updated_at = self.updated_at.date_naive();
        write!(
            f,
            "{0: <5} | {1: <20} | {2: <30} | {3: <30}",
            id, name, created_at, updated_at
        )
    }
}
#[derive(Serialize, Debug)]
pub struct ProjectCreate {
    pub name: String,
    pub description: String,
}
