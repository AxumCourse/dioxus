use dioxus::prelude::*;

use crate::components::frontend::message::{
    Create as FrontIndex, Layout as FrontLayout, View as FrontMessageView,
};
use crate::components::not_found::NotFound;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/")]
        #[layout(FrontLayout)]
        #[route("/")]
        FrontIndex{},
        #[route("/view/:id")]
        FrontMessageView{id:String},
        #[end_layout]
    #[end_nest]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
