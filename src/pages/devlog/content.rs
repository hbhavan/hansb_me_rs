use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    components::menu::*,
    data::markdown::{Markdown, Paragraph},
};

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

#[component]
fn DevLogListingSrv(id: String) -> Element {
    let mut menu = MenuProp { menu_items: vec![] };
    let mut markdown = Markdown::parse_example();

    #[cfg(feature = "server")]
    {
        use crate::server::read_md::get_md_file;

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
        menu = MenuProp::new(&markdown);
    }

    rsx! {
        Menu { prop: menu }
        main {
            class: "md",
            for p in markdown.content.to_vec() {
                {Markdown::render_paragraph(p.clone())}
            }
        }
    }
}
