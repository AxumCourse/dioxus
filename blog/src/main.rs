use dioxus::{logger::tracing::info, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        BlogPost {}
    }
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: u32,
    pub id: u32,
    pub title: String,
    pub body: String,
}
#[component]
fn BlogPost() -> Element {
    let mut post_title = use_signal(|| "".to_string());

    let fetch_post = move |_| async move {
        let resp = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
            .await
            .unwrap()
            .json::<Post>()
            .await
            .unwrap();
        info!("Post: {:#?}", resp);
        post_title.set(resp.title);
    };

    rsx! {
        h1 { "{post_title}" }
        button { class: "px-3 py-1 bg-gray-700 text-white", onclick: fetch_post, "获取数据" }
    }
}
