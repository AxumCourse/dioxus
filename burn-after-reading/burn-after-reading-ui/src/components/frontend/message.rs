use crate::router::Route;
use dioxus::prelude::*;
use lucide_dioxus::CircleCheck as SuccessIcon;

const LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub fn Layout() -> Element {
    rsx! {
        header { class: "axum-container flex items-center gap-x-8 py-6",
            Link { class: "flex items-center gap-x-2", to: Route::FrontIndex {},
                img { src: LOGO, class: "w-10 object-convert" }
                h2 { class: "text-2xl font-normal", "阅后即焚" }
            }
        }
        main { Outlet::<Route> {} }
    }
}

#[component]
pub fn Create() -> Element {
    rsx! {
        div { class: "axum-container my-6 bg-gray-50/70 p-6 space-y-4",
            div { class: " ",
                textarea {
                    class: "w-full ring-1 ring-sky-300 ring-inset focus:ring-sky-500 min-h-96 outline-0 px-3 py-1.5 rounded-md bg-white",
                    placeholder: "在此输入你要发送的信息",
                }
            }
            div { class: "flex flex-col items-start gap-y-2 lg:items-center lg:justify-between lg:flex-row lg:gap-y-0",
                label { class: "grow shrink-0",
                    "密码："
                    input {
                        class: "ring-1 ring-gray-300 ring-inset focus:ring-gray-500  outline-0 px-3 py-1.5 rounded-md bg-white",
                        placeholder: "如不需要密码，请留空",
                    }
                }
                div { class: " shrink-0",
                    button { class: "px-3 py-1.5 bg-gray-900 text-white rounded hover:bg-gray-800",
                        "创建信息"
                    }
                }
            }
        }
    }
}

#[component]
pub fn CreateSuccess(id: String) -> Element {
    rsx! {
        div { class: "fixed inset-0 z-[10] bg-black/75",
            div { class: "absolute w-11/12 lg:w-auto left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded p-6 space-y-4",
                div { class: "flex items-center gap-x-2",
                    div { class: "text-green-600 shrink-0",
                        SuccessIcon { size: 20 }
                    }
                    div { class: "lg:text-lg grow",
                        span { class: "hidden lg:inline", "消息创建成功，" }
                        span { "请将以下网址复制给你要分享的人：" }
                    }
                }
                div { class: "",
                    input {
                        value: "https://foo.barasdfasdfasfasdfasdfasdfasdfasdfasdfasdfsf/view/{id}",
                        readonly: true,
                        class: "font-mono outline-0 ring-1 ring-inset rounded p-3 ring-gray-300 w-full",
                    }
                }

                div { class: "flex justify-end",
                    button { class: "px-3 py-1.5 bg-gray-900 text-white rounded hover:bg-gray-800 text-sm",
                        "完成"
                    }
                }
            }
        }
    }
}

#[component]
pub fn View(id: String) -> Element {
    rsx! {}
}
