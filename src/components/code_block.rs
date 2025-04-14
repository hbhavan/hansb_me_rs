use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CodeBlockProp {
    pub language: PrgLanguage,
    pub text: String,
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum PrgLanguage {
    CSharp,
    Haskell,
    Rust,
    Typescript,
}

#[allow(dead_code)]
impl CodeBlockProp {
    pub fn new(language: PrgLanguage, text: &str) -> Self {
        CodeBlockProp {
            language,
            text: text.to_string(),
        }
    }
}

#[allow(dead_code)]
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
pub fn Code_Block(prop: CodeBlockProp) -> Element {
    rsx! {
        div {
            class: "code-block",
            code {
                language: prop.language.to_string(),
                {prop.text}
            }
        }
    }
}
