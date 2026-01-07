use dioxus::prelude::*;

use crate::{
    components::*,
    data::{markdown::Markdown, menu::MenuMaker},
    pages::projects::content::{get_project_by_id, Project},
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
        let menu = p.desc.to_menu();

        rsx! {
            PageTitle { text: p.title, size: TitleSize::Medium }
            Menu { menu: menu }

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
