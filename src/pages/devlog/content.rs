use dioxus::prelude::*;

use crate::{components::*, data::*};

#[component]
pub fn DevLogListing(id: String) -> Element {
    rsx! {
        if cfg!(feature = "server") {
            DevLogListingSrv { id: id }
        } else {
            div { }
        }
    }
}

#[allow(unused_mut)]
#[component]
fn DevLogListingSrv(id: String) -> Element {
    let mut menu: Vec<MenuItem> = vec![];
    let mut markdown = Markdown::parse_example();

    #[cfg(feature = "server")]
    {
        use crate::{data::menu::MenuMaker, server::read_md::get_md_file};
        use dioxus::{html::mark, logger::tracing::info};

        let path = format!("./store/markdown/{}/{}.md", id, id).to_string();

        let md_text = use_server_future(move || get_md_file(path.clone().into()))?;

        markdown = match md_text() {
            Some(value) => match value {
                Ok(res) => Markdown::new(Paragraph::get_paragraphs(&res)),
                Err(e) => {
                    info!("Error retrieving markdown: {}", e);
                    Markdown::parse_example()
                }
            },
            None => {
                info!("Markdown not found for {}", id);
                Markdown::parse_example()
            }
        };
        menu = markdown.to_menu();
    }

    rsx! {
        Menu { menu: menu }
        main {
            class: "md",
            for p in markdown.content.to_vec() {
                {Markdown::render_paragraph(p.clone())}
            }
        }
    }
}
