use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub dateline: chrono::DateTime<chrono::Local>,
}
