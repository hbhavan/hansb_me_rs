use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            id: "listing",
        }
    }
}

#[component]
fn Listing(id: i32) -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "Hans B. me"}
        }
    }
}
