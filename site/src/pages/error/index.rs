use dioxus::prelude::*;

#[component]
pub fn ErrorPage() -> Element {
    rsx! {
        p { "Please contact Hans about this" }
    }
}
