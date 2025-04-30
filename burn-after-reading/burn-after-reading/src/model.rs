use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{util, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub dateline: chrono::DateTime<chrono::Local>,
}

impl Message {
    pub fn build(content: String, password: String) -> Result<Self> {
        let password = if password.trim().is_empty() {
            "".to_string()
        } else {
            util::hash_pwd(password.trim())?
        };

        Ok(Self {
            id: util::new_id(),
            content,
            password,
            dateline: chrono::Local::now(),
        })
    }

    pub fn if_has_password(&self) -> (bool, &str) {
        if self.password.is_empty() {
            (false, "")
        } else {
            (true, &self.password)
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Admin {
    pub id: String,
    pub username: String,
    pub password: String,
}
