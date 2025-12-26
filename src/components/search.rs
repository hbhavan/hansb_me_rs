use std::fmt::Debug;

use dioxus::prelude::*;

use crate::data::Searchable;

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
