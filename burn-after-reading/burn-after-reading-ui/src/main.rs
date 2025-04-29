use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

// 声明一个全局信号
static COLOR: GlobalSignal<String> = Signal::global(|| "Red".to_string());

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        FavColor {}
    }
}

#[component]
fn FavColor() -> Element {
    // 可以直接在组件中读取全局信号的值
    dioxus::logger::tracing::info!("当前颜色：{}", COLOR);
    rsx!(
        // 可以直接在 rsx 中读取全局信号的值
        h1 { "你喜欢的颜色：{COLOR}" }
        ChangeFavColor {}
    )
}

#[component]
fn ChangeFavColor() -> Element {
    // 修改全局状态
    let handle_change_color = move |_| *COLOR.write() = "Green".into();
    rsx!(
        button { onclick: handle_change_color, "绿色" }
    )
}
