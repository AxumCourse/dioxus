use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Form {}
    }
}

#[component]
fn Form() -> Element {
    let city_list = vec![
        ("gz".to_string(), "广州".to_string()),
        ("sh".to_string(), "上海".to_string()),
        ("sz".to_string(), "深圳".to_string()),
    ];
    let mut city_code = use_signal(|| "gz".to_string());

    rsx!(
        div {
            label { "城市：" }
            select {
                value: "{city_code}",
                onchange: move |e| {
                    city_code.set(e.value());
                },

                for (code , name) in city_list {
                    option { value: "{code}", "{name}" }
                }
            }
        }



        div { "你选择的城市：{city_code}" }
    )
}
