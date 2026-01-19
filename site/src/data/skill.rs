use dioxus::prelude::*;

use crate::components::BadgeItem;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Skill {
    Dotnet,
    Rust,
    React,
    Typescript,
    SQL,
    MongoDB,
    Azure,
}

impl Skill {
    pub fn as_str<'a>(&self) -> &'a str {
        use Skill::*;
        match self {
            Dotnet => ".NET",
            Rust => "Rust",
            React => "React",
            Typescript => "Typescript",
            SQL => "SQL",
            MongoDB => "MongoDB",
            Azure => "Azure"
        }
    }

    pub fn bg_color<'a>(&self) ->  &'a str {
        use Skill::*;
        match self {
            Dotnet => "#A68EBF",
            Rust => "#CE7948",
            React => "#60D9FA",
            Typescript => "#3178C6",
            SQL => "#00BCF2",
            MongoDB => "#5F9E3C",
            Azure => "#769995"
        }
    }
}

impl BadgeItem for Skill {
    fn label(&self) -> String {
        self.as_str().to_string()
    }

    fn icon(&self) -> dioxus::prelude::Asset {
        use Skill::*;
        match self {
            Dotnet => asset!("../../assets/img/icon/skill/dotnet.png"),
            Rust => asset!("../../assets/img/icon/skill/rust.png"),
            React => asset!("../../assets/img/icon/skill/react.png"),
            Typescript => asset!("../../assets/img/icon/skill/typescript.png"),
            SQL => asset!("../../assets/img/icon/skill/sql.png"),
            MongoDB => asset!("../../assets/img/icon/skill/mongodb.png"),
            Azure => asset!("../../assets/img/icon/skill/azure.png")
        }
    }
    
    fn style(&self) -> String {
        format!("color: {bg}", bg = self.bg_color())
    }
}
