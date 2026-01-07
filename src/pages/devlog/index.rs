use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn DevLog() -> Element {
    rsx! {
        DevLogSrv { }
    }
}

#[allow(unused_mut)]
#[component]
fn DevLogSrv() -> Element {
    let mut listings: Vec<Listing> = vec![];

    #[cfg(feature = "server")]
    {
        use crate::{
            data::directory::Directory,
            server::{devlog::get_listings, read_dir::from_dir},
        };
        use dioxus::logger::tracing::info;
        info!("Devlog Server");

        let markdown_listings = use_server_future(move || from_dir(Directory::Markdown))?;

        listings = match markdown_listings() {
            Some(value) => match value {
                Ok(res) => res,
                Err(e) => {
                    info!("Error getting listings: {}", e);
                    Vec::new()
                }
            },
            None => {
                info!("No results returned");
                Vec::new()
            }
        };
    }

    rsx! {
        main {
            id: "dev-log",
            PageTitle { text: "Dev Log", size: TitleSize::Big }
            Listings { listable: listings }
        }
    }
}
