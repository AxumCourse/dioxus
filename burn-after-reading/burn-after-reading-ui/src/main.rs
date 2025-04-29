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
        #[layout(BlogLayout)]
            #[route("/")]
            BlogList {},
            #[route("/:id")]
            BlogDetail { id: u32 },
        #[end_layout]
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
        ol {
            for i in 0..10 {
                li {
                    Link { to: Route::BlogDetail { id: i }, "blog #{i}" }
                }
            }
        }
    }
}
#[component]
fn BlogDetail(id: u32) -> Element {
    rsx! {
        div { "blog detail #{id}" }
    }
}

#[component]
fn BlogLayout() -> Element {
    rsx! {
        h1 { "Blog" }
        ul {
            li {
                Link { to: Route::Home {}, "首页" }
            }
            li {
                Link { to: Route::BlogList {}, "博客列表" }
            }
        }
        Outlet::<Route> {}
    }
}
