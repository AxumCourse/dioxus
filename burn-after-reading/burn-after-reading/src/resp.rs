use axum::Json;
use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct Resp<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> Resp<T> {
    pub fn new(code: i32, msg: String, data: T) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, "OK".to_string(), data)
    }
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl Resp<()> {
    pub fn err(err: Error) -> Self {
        Self::new(-1, err.to_string(), ())
    }

    pub fn void() -> Self {
        Self::ok(())
    }
}

pub fn void() -> Resp<()> {
    Resp::void()
}
pub fn ok<T: Serialize>(data: T) -> Resp<T> {
    Resp::ok(data)
}

pub fn err(err: Error) -> Resp<()> {
    Resp::err(err)
}

pub type JsonResp<T> = Json<Resp<T>>;

#[derive(Serialize)]
pub struct IdResp {
    pub id: String,
}

#[derive(Serialize)]
pub struct AffResp {
    pub aff: u64,
}

pub fn id_resp(id: impl Into<String>) -> Resp<IdResp> {
    ok(IdResp { id: id.into() })
}
