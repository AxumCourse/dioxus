use crate::component::post::{Detail as PostDetail, List as PostList};
use dioxus::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    PostList {},

    #[route("/post/:id")]
    PostDetail { id: u32 },
}
