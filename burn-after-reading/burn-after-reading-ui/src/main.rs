use dioxus::{html::div, prelude::*};

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

        Form {}
    }
}

#[component]
fn Form() -> Element {
    let mut name = use_signal(|| "".to_string());

    rsx!(
        input { value: "{name}", oninput: move |e| name.set(e.value()) }
        if name.read().is_empty() {
            div { "请输入你的名字" }
        } else {
            div { "你的名字是：{name}" }
        }
    )
}
