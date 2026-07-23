use super::ProgrammingLanguage;

impl ProgrammingLanguage {
    pub fn parse<'a>(chars: &'a[char]) -> Self {
        match String::from_iter(chars).to_uppercase().replace(" ", "").as_str() {
            "C++" | "CPP" => ProgrammingLanguage::CPlusPlus,
            "C#" | "CSHARP" => ProgrammingLanguage::CSharp,
            "CSS" => ProgrammingLanguage::CSS,
            "HTML" => ProgrammingLanguage::HTML,
            "RUST" => ProgrammingLanguage::Rust,
            "JS" | "JAVASCRIPT" => ProgrammingLanguage::Javascript,
            "SQL" => ProgrammingLanguage::SQL,
            "TS" | "TYPESCRIPT" => ProgrammingLanguage::Typescript,
            _ => ProgrammingLanguage::None
        }
    }

    pub fn get_alias(&self) -> String {
        use ProgrammingLanguage::*;

        let alias = match self {
            CPlusPlus => "cpp",
            CSharp => "cs",
            CSS => "css",
            HTML => "html",
            Javascript => "js",
            Rust => "rs",
            SQL => "sql",
            Typescript => "ts",
            None => ""
        };

        return String::from(alias)
    }
}
