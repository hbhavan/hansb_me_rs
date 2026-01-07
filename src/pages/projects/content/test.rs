use crate::pages::projects::content::ProjectData;
use crate::data::*;



pub struct TestProject;

impl ProjectData for TestProject {
    fn project_id(&self) -> String {
        "test".into()
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
        "Test".into()
    }
}
