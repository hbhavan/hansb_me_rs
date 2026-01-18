use dioxus::prelude::*;

use crate::{components::*, data::*};

#[component]
pub fn DevLogListing(id: String) -> Element {
    let md_text = use_server_future(move || get_devlog(id.clone()))?;

    let markdown = match md_text() {
        Some(value) => match value {
            Ok(res) => Markdown::from_string(res),
             Err(e) => {
                 info!("Error retrieving markdown: {}", e);
                 Markdown::parse_example()
             }
         },
         None => {
             info!("Markdown not found");
             Markdown::parse_example()
         }
    };
    let menu = markdown.to_menu();

    rsx! {
        Menu { menu }
        main { class: "md",
            for p in markdown.content.to_vec() {
                {Markdown::render_paragraph(p.clone())}
            }
        }
    }
}

#[get("/get_devlog/{id}")]
async fn get_devlog(id: String) -> Result<String, ServerFnError> {
    use dioxus::logger::tracing::info;
    use std::char;
    use std::fs::File;
    use std::io::Read;

    let path = format!("./store/markdown/{}/{}.md", id, id).to_string();

    let mut buf = String::from("");
    info!("Path: {:?}", path);

    match File::open(path).and_then(|mut f| f.read_to_string(&mut buf)) {
        Ok(_) => Ok(buf),
        Err(_) => Err(ServerFnError::new("Could not open file")),
    }
}
