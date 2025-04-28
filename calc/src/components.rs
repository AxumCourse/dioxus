use dioxus::prelude::*;

#[component]
pub fn Info(count: Signal<i32>) -> Element {
    rsx! {
        div { class: "text-center text-2xl", "当前计数：{count}" }
    }
}

#[component]
pub fn Action(mut count: Signal<i32>) -> Element {
    let plus_handler = move |_| count.set(count + 1);
    let sub_handler = move |_| count.set(count - 1);
    rsx! {
        div { class: "flex justify-center items-center gap-x-4",
            button {
                class: "px-3 py-1 bg-gray-600 text-white rounded cursor-pointer hover:bg-gray-700",
                onclick: sub_handler,
                "-1"
            }
            button {
                class: "px-3 py-1 bg-blue-600 text-white rounded cursor-pointer hover:bg-blue-700",
                onclick: plus_handler,
                "+1"
            }
        }
    }
}

#[component]
pub fn CalcView() -> Element {
    let count = use_signal(|| 0);

    rsx! {
        div { class: "absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 p-6 rounded-lg border shadow-lg space-y-6 bg-gray-100 min-w-96",
            Info { count }
            Action { count }
        }
    }
}
