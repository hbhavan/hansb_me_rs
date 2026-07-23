use dioxus::prelude::*;
use writ::data::markdown::*;
use crate::{utils::render::Render};

#[component]
pub fn DevLogContent(devlog_id: i32) -> Element {
    let md_text = use_server_future(move || get_devlog(devlog_id))?;

    let markdown = match md_text() {
        Some(value) => match value {
            Ok(res) => MarkdownContent::from_string(&res),
             Err(e) => {
                 info!("Error retrieving markdown: {}", e);
                 MarkdownContent::parse_example()
             }
         },
         None => {
             info!("Markdown not found");
             MarkdownContent::parse_example()
         }
    };
    //let menu = markdown.to_menu();

    rsx! {
        //Menu { menu }
        {markdown.render()}
    }
}

#[get("/get_devlog/{id}")]
async fn get_devlog(id: i32) -> Result<String, ServerFnError> {
    use services::api::devlog;

    let markdown = devlog::get_devlog_by_id(id).await
        .map_err(|_| ServerFnError::new("Could not get content"));

    markdown
}
