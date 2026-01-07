use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum PrgLanguage {
    CSharp,
    Haskell,
    Rust,
    Typescript,
}

impl PrgLanguage {
    pub fn to_string(&self) -> &str {
        use PrgLanguage::*;
        match self {
            CSharp => "C#",
            Haskell => "Haskell",
            Rust => "Rust",
            Typescript => "Typescript",
        }
    }
}

#[component]
pub fn Code_Block(language: PrgLanguage, text: String) -> Element {
    rsx! {
        div {
            class: "code-block",
            code {
                language: language.to_string(),
                {text}
            }
        }
    }
}
