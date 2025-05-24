use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        main {
            h1 { "About me" }
        }
    }
}
