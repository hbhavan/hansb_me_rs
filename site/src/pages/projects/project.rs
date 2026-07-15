use dioxus::prelude::*;

use crate::{pages::projects::content::{get_project_by_id}, utils::render::Render};

#[component]
pub fn ProjectContent(id: i32) -> Element {
    let project = get_project_by_id(id);

    if let Some(p) = project {
        //let menu = p.desc.to_menu();

        rsx! {
            div { class: "post-content",
                div {
                    //Menu { menu }
                }
                match p.content {
                    Some(c) => c,
                    None => rsx! {
                        article { {p.desc.render()} }
                    },
                }
                div {}
            }
        }
    } else {
        rsx! {
            {"Project not found"}
        }
    }
}
