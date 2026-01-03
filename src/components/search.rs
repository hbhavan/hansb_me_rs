use dioxus::prelude::*;

#[component]
pub fn Search() -> Element {
    rsx! {
        div {
            class: "search-group",
            label { class: "search-label", {"Search"} }
            input {
                class: "search",
                placeholder: "Enter text here",
            }
        }
    }
}
