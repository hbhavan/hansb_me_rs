use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TitleProp {
    pub size: TitleSize,
    pub text: String,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum TitleSize {
    Big,
    MedBig,
    Medium,
    MedSmall,
    Small,
    Tiny,
}

impl TitleProp {
    pub fn new(size: TitleSize, text: &str) -> Self {
        TitleProp {
            size,
            text: text.to_string(),
        }
    }
}

#[component]
pub fn Title(prop: TitleProp) -> Element {
    rsx! {
        Header {
            text: prop.text,
            size: prop.size
        }
    }
}

#[component]
fn Header(text: String, size: TitleSize) -> Element {
    match size {
        TitleSize::Big => rsx! { h1 { class: "title", {text} } },
        TitleSize::MedBig => rsx! { h2 { class: "title", {text} } },
        TitleSize::Medium => rsx! { h3 { class: "title", {text} } },
        TitleSize::MedSmall => rsx! { h4 { class: "title", {text} } },
        TitleSize::Small => rsx! { h5 { class: "title", {text} } },
        TitleSize::Tiny => rsx! { h6 { class: "title", {text} } },
    }
}
