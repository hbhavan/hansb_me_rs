use dioxus::prelude::*;

#[component]
pub fn Section(text: String) -> Element {
    rsx! {
        section {
            class: "section",
            {text}
        }
    }
}
