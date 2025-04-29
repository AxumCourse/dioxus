use dioxus::prelude::*;
use lucide_dioxus::CircleEllipsis as Icon;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/").to_string());
    rsx! {
        div { class: "absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-white p-6 rounded-md shadow-md flex justify-start items-center gap-x-2 text-lg",
            div { class: "text-orange-800",
                Icon { size: 22 }
            }
            div {
                span { "你访问的页面" }
                span { class: "font-mono mx-1 underline decoration-dotted underline-offset-4 text-orange-800",
                    "{path}"
                }
                span { "不存在" }
            }
        }
    }
}
