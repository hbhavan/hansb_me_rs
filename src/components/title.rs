use dioxus::prelude::*;

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

#[component]
pub fn PageTitle(text: String, size: TitleSize) -> Element {
    rsx! {
        Header {
            text: text,
            size: size
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
