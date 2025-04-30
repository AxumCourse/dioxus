use std::collections::HashMap;

use crate::{api, http, router::Route};
use dioxus::prelude::*;
use lucide_dioxus::{CircleCheck as SuccessIcon, Key as PasswordIcon};

const LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub fn Layout() -> Element {
    rsx! {
        header { class: "axum-container flex items-center gap-x-8 py-6",
            a { class: "flex items-center gap-x-2", href: "/",
                img { src: LOGO, class: "w-10 object-convert" }
                h2 { class: "text-2xl font-normal", "阅后即焚" }
            }
        }
        main { class: "axum-container my-6 bg-gray-50/70 p-6 space-y-4", Outlet::<Route> {} }
    }
}

#[component]
pub fn Create() -> Element {
    let mut created_url = use_signal(|| "".to_string());
    let mut msg_content = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let create_handler = move |_| async move {
        if (*msg_content.read()).is_empty() {
            return;
        }
        let mut payload = HashMap::new();
        payload.insert("content", (*msg_content.read()).clone());
        payload.insert("password", (*password.read()).clone());
        let r = http::new_post("/message")
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json::<api::Resp<api::CreateMessageResp>>()
            .await
            .unwrap();
        if let Some(data) = r.data {
            created_url.set(data.url);
        }
        msg_content.set("".to_string());
        password.set("".to_string());
    };

    rsx! {
        div { class: " ",
            textarea {
                class: "w-full ring-1 ring-sky-300 ring-inset focus:ring-sky-500 min-h-96 outline-0 px-3 py-1.5 rounded-md bg-white",
                value: "{msg_content}",
                oninput: move |e| { msg_content.set(e.value()) },
                placeholder: "在此输入你要发送的信息",
            }
        }
        div { class: "flex flex-col items-start gap-y-2 lg:items-center lg:justify-between lg:flex-row lg:gap-y-0",
            label { class: "grow shrink-0",
                "密码："
                input {
                    class: "ring-1 ring-gray-300 ring-inset focus:ring-gray-500  outline-0 px-3 py-1.5 rounded-md bg-white",
                    r#type: "password",
                    value: "{password}",
                    oninput: move |e| { password.set(e.value()) },
                    placeholder: "如不需要密码，请留空",
                }
            }
            div { class: " shrink-0",
                button {
                    class: "px-3 py-1.5 bg-gray-900 text-white rounded hover:bg-gray-800",
                    onclick: create_handler,
                    "创建信息"
                }
            }
        }
        if !(*created_url.read()).is_empty() {
            CreateSuccess { url: (*created_url.read()).clone(), created_url }
        }
    }
}

#[component]
pub fn CreateSuccess(url: String, created_url: Signal<String>) -> Element {
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
                        value: "{url}",
                        readonly: true,
                        class: "font-mono outline-0 ring-1 ring-inset rounded p-3 ring-gray-300 w-full",
                    }
                }

                div { class: "flex justify-end",
                    button {
                        class: "px-3 py-1.5 bg-gray-900 text-white rounded hover:bg-gray-800 text-sm",
                        onclick: move |_| { created_url.set("".to_string()) },
                        "完成"
                    }
                }
            }
        }
    }
}

#[derive(serde::Serialize, Default)]
pub struct ViewParam {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[component]
pub fn View(id: String) -> Element {
    let param = use_signal(|| ViewParam {
        id: id.clone(),
        password: None,
    });

    let resp = use_resource(move || async move {
        http::new_post("/message/view")
            .json(&*param.read())
            .send()
            .await
            .unwrap()
            .json::<api::Resp<api::MessageResp>>()
            .await
    });
    match &*resp.read_unchecked() {
        Some(Ok(r)) => {
            dioxus::logger::tracing::debug!("{:#?}", r);
            if let Some(data) = &r.data {
                if data.need_password {
                    rsx! {
                        div { class: "text-red-600", "该消息需要密码，请先输入密码" }
                        PasswordDialogForView { id: id.clone(), resp, param }
                    }
                } else {
                    if let Some(msg) = &data.message {
                        rsx! {
                            pre { class: "font-sans", "{msg.content}" }
                        }
                    } else {
                        rsx!(
                            div { class: "text-red-600", "信息不存在或已过期" }
                        )
                    }
                }
            } else {
                rsx! {
                    div { class: "text-red-600", "{r.msg}" }
                }
            }
        }
        Some(Err(e)) => {
            dioxus::logger::tracing::debug!("{:#?}", e);
            rsx! {
                div { class: "text-red-600", "出错了：{e}" }
            }
        }
        None => {
            rsx! {
                div { "Loading..." }
            }
        }
    }
}
#[component]
pub fn PasswordDialogForView(
    id: String,
    resp: Resource<Result<api::Resp<api::MessageResp>, reqwest::Error>>,
    param: Signal<ViewParam>,
) -> Element {
    let mut pwd = use_signal(|| "".to_string());
    rsx! {
        div { class: "fixed inset-0 z-[10] bg-gray-100",
            div { class: "absolute w-11/12 lg:w-auto left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded p-6 space-y-4 border border-gray-200 shadow-md",
                div { class: "flex items-center gap-x-2",
                    div { class: "shrink-0",
                        PasswordIcon { size: 20 }
                    }
                    div { class: "lg:text-lg grow", "该消息需要密码：" }
                }
                div { class: "",
                    input {
                        value: "{pwd}",
                        r#type: "password",
                        placeholder: "请输入访问密码",
                        oninput: move |e| { pwd.set(e.value()) },
                        class: "font-mono outline-0 ring-1 ring-inset rounded px-3 py-1.5 ring-gray-300 w-full",
                    }
                }

                div {
                    class: "flex justify-end",
                    onclick: move |_| {
                        let id = param.read().id.clone();
                        param
                            .set(ViewParam {
                                id,
                                password: Some((*pwd.read()).clone()),
                            });
                        resp.restart();
                    },
                    button { class: "px-3 py-1.5 bg-gray-900 text-white rounded hover:bg-gray-800 text-sm",
                        "提交"
                    }
                }
            }
        }
    }
}
