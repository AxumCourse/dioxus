use dioxus::prelude::*;

use crate::{component::loading::Loading, model};

#[component]
pub fn List() -> Element {
    let posts = use_resource(|| async move {
        reqwest::get("https://jsonplaceholder.typicode.com/posts")
            .await
            .unwrap()
            .json::<Vec<model::Post>>()
            .await
    });
    match &*posts.read_unchecked() {
        Some(Ok(ps)) => rsx! {
            ul { class: "list-disc list-inside space-y-2",
                for post in ps {
                    li {
                        a {
                            class: "hover:text-red-600 capitalize",
                            href: format!("/post/{}", post.id),
                            "{post.title}"
                        }
                    }
                }
            }
        },
        Some(Err(e)) => rsx! {
            div { "获取博文数据出错：{e}" }
        },
        None => rsx! {
            Loading {}
        },
    }
}

#[component]
pub fn Detail(id: u32) -> Element {
    let post = use_resource(move || async move {
        reqwest::get(format!("https://jsonplaceholder.typicode.com/posts/{}", id))
            .await
            .unwrap()
            .json::<model::Post>()
            .await
    });

    match &*post.read_unchecked() {
        Some(Ok(p)) => rsx! {
            div { class: "space-y-4",
                h1 { class: "text-xl font-semibold capitalize", "{p.title}" }
                p { class: "text-lg", "{p.body}" }
            }
        },
        Some(Err(e)) => rsx! {
            div { "获取博文数据出错：{e}" }
        },
        None => rsx! {
            Loading {}
        },
    }
}
