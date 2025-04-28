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

        CalcView {}
    }
}

#[component]
fn CalcView() -> Element {
    let plus_handler = move |e| info!("click {e:?}");
    rsx! {
        div { "当前计数：123" }
        div {
            button { "-1" }
            button { onclick: plus_handler, "+1" }
        }
    }
}
