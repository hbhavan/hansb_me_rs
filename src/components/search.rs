
use dioxus::prelude::*;


#[component]
pub fn Search() -> Element {
    rsx! {
        //label { class: "search-label", {"Search"} }
        input {
            class: "search",
            placeholder: "Enter text here",
        }
    }
}
