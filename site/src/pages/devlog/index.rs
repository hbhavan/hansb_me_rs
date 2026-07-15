use dioxus::prelude::*;

use crate::pages::devlog::listings::DevLogListings;

#[component]
pub fn DevLog() -> Element {
    rsx! {
        div { DevLogListings {} }
    }
}

