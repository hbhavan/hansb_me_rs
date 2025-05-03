use dioxus::{logger::tracing::info, prelude::*};

use crate::{
    components::{
        listing::{Listing, ListingProp},
        navbar::Route,
        title::*,
    },
    data::directory::Directory,
    server::read_dir::from_dir,
};

#[component]
pub fn DevLog() -> Element {
    let title = TitleProp::new(TitleSize::Big, "Dev Log");

    let markdown_listings = use_server_future(move || from_dir(Directory::Markdown))?;

    let listings = match markdown_listings() {
        Some(value) => match value {
            Ok(res) => {
                ListingProp::from_dir(res, |id: &str| Route::DevLogListing { id: id.to_string() })
            }
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

    rsx! {
        main {
            id: "dev-log",
            Title { prop: title }
            ul {
                for listing in listings {
                    li {
                        Listing { prop: listing }
                    }
                }
            }
        }
    }
}
