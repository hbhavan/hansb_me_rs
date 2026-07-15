use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Section { text: "Full-Stack Software Development" }
    }
}
