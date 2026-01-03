use components::navbar::Route;
use dioxus::prelude::*;

mod components;
mod data;
mod pages;
#[cfg(feature = "server")]
mod server;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const SITE_CSS: Asset = asset!("/assets/css/site.css");
const COMPONENT_CSS: Asset = asset!("/assets/css/components.css");
const CUSTOM_CSS: Asset = asset!("/assets/css/custom.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: SITE_CSS }
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }
        document::Link { rel: "stylesheet", href: CUSTOM_CSS }

        Router::<Route> { }
    }
}
