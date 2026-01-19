use dioxus::prelude::*;

use writ::utils::menu::*;
use crate::{
    components::*,
    pages::projects::content::{Project, get_project_by_id}, utils::render::Render,
};

#[component]
pub fn ProjectContent(id: String) -> Element {
    let project = get_project_by_id(id);

    rsx! {
        main {
            ProjectRender { project }
        }
    }
}

#[component]
fn ProjectRender(project: Option<Project>) -> Element {
    if let Some(p) = project {
        let menu = p.desc.to_menu();

        rsx! {
            PageTitle { text: p.title, size: TitleSize::Medium }
            Menu { menu }

            {p.desc.render()}
        }
    } else {
        rsx! {
            {"Project not found"}
        }
    }
}
