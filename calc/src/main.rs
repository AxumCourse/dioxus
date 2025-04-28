use dioxus::prelude::*;

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

        CalcView {}
    }
}

#[component]
fn CalcView() -> Element {
    let mut count = use_signal(|| 0);
    let plus_handler = move |_| count.set(count + 1);
    let sub_handler = move |_| count.set(count - 1);
    rsx! {
        div { "当前计数：{count}" }
        div {
            button { onclick: sub_handler, "-1" }
            button { onclick: plus_handler, "+1" }
        }
    }
}
