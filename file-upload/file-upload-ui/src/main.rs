use std::sync::Arc;

use dioxus::{html::FileEngine, logger::tracing::info, prelude::*};
use reqwest::multipart;

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
        UploadFile {}
    }
}

#[component]
pub fn UploadFile() -> Element {
    let read_files = move |fe: Arc<dyn FileEngine>| async move {
        let files = fe.files();
        for (idx, filename) in files.iter().enumerate() {
            let buf = fe.read_file(&filename).await.unwrap();
            info!("file {filename} has {} bytes", buf.len());

            let part = multipart::Part::bytes(buf).file_name(filename.clone());
            let form = multipart::Form::new().part(format!("img_{}", idx), part);
            reqwest::Client::new()
                .post("http://172.29.101.146:9527")
                .multipart(form)
                .send()
                .await
                .unwrap();
        }
    };
    let upload_file = move |e: FormEvent| async move {
        if let Some(fe) = e.files() {
            read_files(fe).await;
        }
    };
    rsx! {
        div {
            input { r#type: "file", multiple: true, onchange: upload_file }
        }
    }
}
