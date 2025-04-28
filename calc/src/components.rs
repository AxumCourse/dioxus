use dioxus::prelude::*;

#[component]
pub fn Info(count: Signal<i32>) -> Element {
    rsx! {
        div { "当前计数：{count}" }
    }
}

#[component]
pub fn Action(mut count: Signal<i32>) -> Element {
    let plus_handler = move |_| count.set(count + 1);
    let sub_handler = move |_| count.set(count - 1);
    rsx! {
        div {
            button { onclick: sub_handler, "-1" }
            button { onclick: plus_handler, "+1" }
        }
    }
}

#[component]
pub fn CalcView() -> Element {
    let count = use_signal(|| 0);

    rsx! {
        Info { count }
        Action { count }
    }
}
