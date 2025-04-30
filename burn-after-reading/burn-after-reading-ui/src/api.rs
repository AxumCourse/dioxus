use serde::Deserialize;

use crate::model;

#[derive(Deserialize, Debug)]
pub struct Resp<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Deserialize, Debug)]
pub struct MessageResp {
    pub need_password: bool,
    #[serde(flatten)]
    pub message: Option<model::Message>,
}

#[derive(Deserialize, Debug)]
pub struct CreateMessageResp {
    pub id: String,
    pub url: String,
}
