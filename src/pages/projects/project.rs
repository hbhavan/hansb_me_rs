use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
    components::title::{PageTitle, TitleSize},
    data::markdown::Markdown,
    pages::projects::content::{
        blank::Blank, get_project_by_id, projects, Project, ProjectType, Status,
    },
};

#[component]
pub fn ProjectContent(id: String) -> Element {
    let project = get_project_by_id(id);

    rsx! {
        main {
            ProjectRender { project: project }
        }
    }
}

#[component]
fn ProjectRender(project: Option<Project>) -> Element {
    if let Some(p) = project {
        rsx! {
            PageTitle { text: p.title, size: TitleSize::Medium }

            for p in p.desc.content.to_vec() {
                {Markdown::render_paragraph(p.clone())}
            }
        }
    } else {
        rsx! {
            {"Project not found"}
        }
    }
}
