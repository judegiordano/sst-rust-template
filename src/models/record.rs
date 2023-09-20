use mongoose::{bson::DateTime, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[serde(rename = "_id")]
    pub id: String,
    pub payload: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
impl Default for Record {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            payload: std::string::String::default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
impl Model for Record {}
