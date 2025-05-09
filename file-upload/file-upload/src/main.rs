use axum::{
    extract::{DefaultBodyLimit, Multipart},
    routing::post,
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9527").await.unwrap();

    let app = Router::new()
        .route("/", post(upload_file))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            512 * 1024 * 1024, // 512mb
        ))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any),
        );

    axum::serve(listener, app).await.unwrap();
}

async fn upload_file(mut mp: Multipart) {
    while let Some(field) = mp.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let resp = format!(
            "字段名：`{name}`，文件名：`{file_name}`，类型：`{content_type}`，文件大小： {} 字节",
            data.len()
        );

        println!("{resp}");
    }
}
