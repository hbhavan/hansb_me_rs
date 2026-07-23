use crate::utils::render::Render;

use super::*;

pub struct SetGame;

impl ProjectData for SetGame {
    fn project_id(&self) -> String {
        String::from("set")
    }

    fn title(&self) -> String {
        String::from("Set")
    }

    fn link(&self) -> Option<String> {
        None
    }

    fn project_type(&self) -> ProjectType {
        ProjectType::Personal
    }

    fn skills(&self) -> Vec<Skill> {
        use Skill::*;
        vec![Rust]
    }

    fn status(&self) -> Status {
        Status::concluded((2026, 7), (2026, 7))
    }

    fn desc(&self) -> String {
        String::from("This is set")
    }

    fn render_project(&self) -> Option<Element> {
        Some(self.render())
    }
}
