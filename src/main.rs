use components::navbar::Route;
use dioxus::prelude::*;

mod components;
mod data;
mod pages;
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

// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
