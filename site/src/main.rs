use crate::layout::Route;
use dioxus::prelude::*;
use dotenv::dotenv;


pub mod components;
pub mod data;
pub mod layout;
pub mod pages;
pub mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const SITE_CSS: Asset = asset!("/assets/css/site.css");
const COMPONENT_CSS: Asset = asset!("/assets/css/components.css");
const CUSTOM_CSS: Asset = asset!("/assets/css/custom.css");

pub fn main() {
    dotenv().ok();

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: SITE_CSS }
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }
        document::Link { rel: "stylesheet", href: CUSTOM_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&family=Space+Mono:ital,wght@0,400;0,700;1,400;1,700&display=swap",
        }

        Router::<Route> {}
    }
}
