use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod main_layout;


use crate::layout::main_layout::MainLayout;
use crate::pages::*;

#[derive(Debug, Clone, Routable, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home,

    #[route("/devlog")]
    DevLog,
    #[route("/devlog/listing/:id")]
    DevLogListing { id: i32 },

    #[route("/projects")]
    Projects,
    #[route("/projects/:id")]
    ProjectContent { id: i32 },

    #[route("/about")]
    About,

    #[route("/error")]
    ErrorPage 
}

impl Route {
    pub fn title(&self) -> Option<String> {
        use Route::*;

        match self {
            Home => Some(String::from("Hans Bhavan")),
            DevLog => Some(String::from("Dev Log")),
            Projects => Some(String::from("Projects")),
            About => Some(String::from("About")),
            ErrorPage => Some(String::from("Error")),
            _ => None
        }
    }
}
