mod err;
pub mod handler;
pub mod model;

pub use err::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;
