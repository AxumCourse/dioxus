use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_max_conns: u32,
    pub web_addr: String,
    pub delete_interval: u32,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let c = ::config::Config::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize()?;
        Ok(c)
    }
}
