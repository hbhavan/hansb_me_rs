use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum HeadingSize {
    Big,
    MedBig,
    Medium,
    MedSmall,
    Small,
    Tiny,
}

#[component]
pub fn Heading(text: String, size: HeadingSize) -> Element {
    match size {
        HeadingSize::Big => rsx! {
            h1 { class: "heading", {text} }
        },
        HeadingSize::MedBig => rsx! {
            h2 { class: "heading", {text} }
        },
        HeadingSize::Medium => rsx! {
            h3 { class: "heading", {text} }
        },
        HeadingSize::MedSmall => rsx! {
            h4 { class: "heading", {text} }
        },
        HeadingSize::Small => rsx! {
            h5 { class: "heading", {text} }
        },
        HeadingSize::Tiny => rsx! {
            h6 { class: "heading", {text} }
        },
    }
}
