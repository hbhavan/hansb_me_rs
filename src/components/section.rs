use std::sync::Arc;

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionProp {
    pub text: Arc<str>,
}

impl SectionProp {
    pub fn new(text: &str) -> Self {
        SectionProp { text: text.into() }
    }
}

#[component]
pub fn Section(prop: SectionProp) -> Element {
    rsx! {
        section {
            class: "section",
            {prop.text}
        }
    }
}
