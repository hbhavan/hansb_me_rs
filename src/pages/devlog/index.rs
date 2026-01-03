use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    components::{
        listing::{Listing, ListingProp},
        navbar::Route,
        title::*,
    },
    data::directory::Directory,
};

#[component]
pub fn DevLog() -> Element {
    rsx! {
        DevLogSrv { }
    }
}

#[component]
fn DevLogSrv() -> Element {
    let mut listings: Vec<ListingProp> = vec![];
    #[cfg(feature = "server")]
    {
        use crate::server::{devlog::get_listings, read_dir::from_dir};
        info!("Devlog Server");

        let markdown_listings = use_server_future(move || from_dir(Directory::Markdown))?;

        let lst = use_server_future(move || get_listings())?;

        if let Some(_) = lst() {
            info!("Got a listing");
        }

        listings = match markdown_listings() {
            Some(value) => match value {
                Ok(res) => ListingProp::from_dir(res, |id: &str| Route::DevLogListing {
                    id: id.to_string(),
                }),
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
            ul {
                for listing in listings {
                    li { Listing { prop: listing } }
                }
            }
        }
    }
}
