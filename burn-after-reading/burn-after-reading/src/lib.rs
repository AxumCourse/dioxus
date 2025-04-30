pub mod config;
pub mod db;
mod err;
pub mod form;
pub mod frontend;
pub mod model;
pub mod resp;
pub mod util;

use std::sync::Arc;

pub use err::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub cfg: Arc<config::Config>,
}

pub type ArcAppState = std::sync::Arc<AppState>;
