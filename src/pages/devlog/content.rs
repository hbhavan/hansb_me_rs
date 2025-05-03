use dioxus::{html::mark, logger::tracing::info, prelude::*};

use crate::{
    components::menu::*,
    data::{
        markdown::{Markdown, Paragraph},
        seq::Seq,
    },
    server::read_md::get_md_file,
};

#[component]
pub fn DevLogListing(id: String) -> Element {
    let path = format!("./store/markdown/{}/{}.md", id, id).to_string();

    let md_text = use_server_future(move || get_md_file(path.clone().into()))?;

    let markdown = match md_text() {
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

    let menu = MenuProp::new(&markdown);

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
