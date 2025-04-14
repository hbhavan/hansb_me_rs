use dioxus::{logger::tracing::info, prelude::*};

use crate::{
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

    let paragraphs = match md_text() {
        Some(value) => match value {
            Ok(res) => Paragraph::get_paragraphs(&res),
            Err(e) => {
                info!("Error retrieving markdown: {}", e);
                Seq::from_vec(Markdown::parse_example())
            }
        },
        None => {
            info!("Markdown not found for {}", id);
            Seq::from_vec(Markdown::parse_example())
        }
    };

    rsx! {
        for p in paragraphs.to_vec() {
            {Markdown::render_paragraph(p.clone())}
        }
    }
}
