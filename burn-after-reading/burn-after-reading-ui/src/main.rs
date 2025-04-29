use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

// 全局共享状态包装器
#[derive(Clone, Copy)]
struct ColorState(Signal<String>);

#[component]
fn App() -> Element {
    let color = use_signal(|| "Red".to_string());
    // 全局状态提供者
    use_context_provider(|| ColorState(color));

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
        ChangeFavColor {}
    )
}

#[component]
fn ChangeFavColor() -> Element {
    // 修改全局状态
    let handle_change_color = move |_| consume_context::<ColorState>().0.set("Blue".into());
    rsx!(
        button { onclick: handle_change_color, "蓝色" }
    )
}
