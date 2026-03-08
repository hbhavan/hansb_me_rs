use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Heading { text: "Hans Bhavan", size: HeadingSize::Medium }

        Section { text: "Full-Stack Software Development" }
    }
}
