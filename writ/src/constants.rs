#[derive(Clone, Debug, PartialEq)]
pub enum ProgrammingLanguage {
    CPlusPlus,
    CSharp,
    CSS,
    HTML,
    Rust,
    Javascript,
    SQL,
    Typescript,
    None
}

pub mod programming_language;

pub use programming_language::*;
