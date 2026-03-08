use dioxus::prelude::*;
use writ::data::markdown::*;
use writ::utils::menu::*;
use crate::{components::*, utils::render::Render};

#[component]
pub fn DevLogListing(id: i32) -> Element {
    let md_text = use_server_future(move || get_devlog(id))?;

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
        main { class: "md", {markdown.render()} }
    }
}

#[get("/get_devlog/{id}")]
async fn get_devlog(id: i32) -> Result<String, ServerFnError> {
    use services::api::devlog;

    let markdown = devlog::get_devlog_by_id(id).await
        .map_err(|_| ServerFnError::new("Could not get content"));

    markdown
}
