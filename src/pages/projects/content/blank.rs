use super::*;

pub struct Blank;

impl ProjectData for Blank {
    fn project_id(&self) -> String {
        String::from("")
    }

    fn title(&self) -> String {
        String::from("Project not found")
    }

    fn project_type(&self) -> super::ProjectType {
        super::ProjectType::Misc
    }

    fn status(&self) -> super::Status {
        Status::Unknown
    }

    fn skills(&self) -> Vec<Skillset> {
        vec![]
    }

    fn desc(&self) -> String {
        String::from("")
    }
}
