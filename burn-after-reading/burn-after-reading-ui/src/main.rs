use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[nest("/blog")]
        #[route("/")]
        BlogList {},
        #[route("/:id")]
        BlogDetail { id: u32 },
    #[end_nest]
    
    #[route("/")]
    Home {},
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
    rsx!(
        div { "home" }
    )
}

#[component]
fn BlogList() -> Element {
    rsx! {
        div { "blog list" }
    }
}
#[component]
fn BlogDetail(id: u32) -> Element {
    rsx! {
        div { "blog detail #{id}" }
    }
}
