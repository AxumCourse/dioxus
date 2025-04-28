use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "text-red-600 text-xl", "首页" }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div { class: "text-blue-600 text-xl", "关于" }
    }
}
