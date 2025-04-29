use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

// 全局共享状态包装器
#[derive(Clone)]
struct ColorState(String);

#[component]
fn App() -> Element {
    // 全局状态提供者
    use_context_provider(|| ColorState("Red".into()));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        FavColor {}
    }
}

#[component]
fn FavColor() -> Element {
    // 从上下文中消费全局状态
    let color = use_context::<ColorState>();

    rsx!(
        h1 { "你喜欢的颜色：{color.0}" }
    )
}
