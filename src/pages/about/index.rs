use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            h1 { "About me" }
        }
    }
}
