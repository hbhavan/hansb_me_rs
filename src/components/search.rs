use dioxus::prelude::*;

//use crate::data::searchable::Searchable;

#[component]
pub fn Search(on_search_change: EventHandler<Event<FormData>>) -> Element {
    rsx! {
        div {
            class: "search-group",
            label { class: "search-label", {"Search"} }
            input {
                class: "search",
                placeholder: "Enter text here",
                oninput: move |e| on_search_change.call(e)
            }
        }
    }
}
