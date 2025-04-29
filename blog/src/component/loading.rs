use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx!(
        div { "正在加载..." }
    )
}
