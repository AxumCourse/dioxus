use blog::route::Route;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const LOGO_IMG: Asset = asset!("/assets/logo.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "bg-gray-100 min-h-screen",
            header { class: "container mx-auto flex justify-start items-end gap-x-8 py-6",
                a { class: "flex items-center gap-x-2", href: "/",
                    img { src: LOGO_IMG, class: "w-8 object-convert" }
                    h2 { class: "text-2xl font-bold", "博客" }
                }
                nav { class: "",
                    ul {
                        li {
                            a {
                                class: "text-xl hover:text-red-800",
                                href: "/",
                                "博文"
                            }
                        }
                    }
                }
            }
            main { class: "container mx-auto p-6 bg-white", Router::<Route> {} }
        }
    }
}
