use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Create {
    #[validate(length(min = 1, message = "信息不能为空"))]
    pub content: String,
    pub password: String,
}
#[derive(Deserialize, Validate)]
pub struct Read {
    #[validate(length(min = 20, max = 20, message = "ID错误"))]
    pub id: String,
    pub password: Option<String>,
}
