use axum::{extract::Path, response::Html};
use dioxus::prelude::*;
use dioxus_ssr::render_element;

use crate::{model, Result};

async fn http_get<T: serde::de::DeserializeOwned>(url: &str) -> Result<T> {
    let resp = reqwest::get(url).await?.json::<T>().await?;
    Ok(resp)
}

pub async fn list() -> Result<Html<String>> {
    let users = http_get::<Vec<model::User>>("https://jsonplaceholder.typicode.com/users").await?;
    Ok(Html(render_element(rsx! {
        h1 { "用户列表" }
        ul {
            for user in users {
                li {
                    a { href: "/user/{user.id}", "{user.name}" }
                }
            }
        }
    })))
}

pub async fn detail(Path(id): Path<u32>) -> Result<Html<String>> {
    let user = http_get::<model::User>(
        format!("https://jsonplaceholder.typicode.com/users/{id}").as_str(),
    )
    .await?;
    Ok(Html(render_element(rsx! {
        h1 { "用户详情" }

        h2 { "基本信息" }
        ul {
            li { "id: {user.id}" }
            li { "name: {user.name}" }
            li { "username: {user.username}" }
            li { "email: {user.email}" }
            li { "phone: {user.phone}" }
            li { "website: {user.website}" }
        }

        h2 { "地址信息" }
        ul {
            li { "street: {user.address.street}" }
            li { "suite: {user.address.suite}" }
            li { "city: {user.address.city}" }
            li { "zipcode: {user.address.zipcode}" }
            li { "geo: {user.address.geo.lat}, {user.address.geo.lng}" }
        }

        h2 { "公司信息" }
        ul {
            li { "name: {user.company.name}" }
            li { "catchPhrase: {user.company.catch_phrase}" }
            li { "bs: {user.company.bs}" }
        }

        div {
            a { href: "/", "返回" }
        }
    })))
}
