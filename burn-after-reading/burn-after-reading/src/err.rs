use axum::response::IntoResponse;

use crate::resp;

#[derive(Debug)]
pub struct Error(anyhow::Error);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        resp::err(self).to_json().into_response()
    }
}
