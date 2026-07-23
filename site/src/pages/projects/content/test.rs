use crate::pages::projects::content::ProjectData;
use crate::data::*;



pub struct TestProject;

impl ProjectData for TestProject {
    fn project_id(&self) -> String {
        String::from("test")
    }

    fn title(&self) -> String {
        "Test".into()
    }

    fn skills(&self) -> Vec<Skill> {
        use Skill::*;
        vec![
            Dotnet,
            Rust,
            React,
            Typescript,
            SQL,
            MongoDB,
            Azure,
        ]
    }

    fn status(&self) -> super::Status {
        super::Status::Unknown
    }

    fn project_type(&self) -> super::ProjectType {
        super::ProjectType::Misc
    }

    fn desc(&self) -> String {
        String::from("Parse example
            \n*Italic*
            \n**Bold**
            \n***Bold Italic***
            \n~Struck~
            \n_Italic_
            \n__Bold__
            \n___Bold Italic___
            \n- Item *1*
            \n- Item ~2~
            \n- Item _3_
            \n[link title](link source)
            \n![img title](img source)
            \n> Block Quote
            \n> Block Quote
            \n> Block Quote
            \nVarious *Text* ~Example~ Line
            \nSample **text** sample ___text___ sample
            \n1. Item A
            \n2. Item B
            \n3. Item C
            \n===
            \n# Test
            \n## *Test*
            \n### ~Test~
            \n#### Test
            \n##### Test
            \n###### Test")
    }
}
