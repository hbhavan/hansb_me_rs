use dioxus::prelude::*;

use crate::components::{
    listing::{Directory, Listing, ListingProp},
    navbar::Route,
    title::*,
};

#[component]
pub fn DevLog() -> Element {
    let title = TitleProp::new(TitleSize::Big, "Dev Log");

    let listings = ListingProp::from_dir(Directory::Markdown, |id: &str| Route::DevLogListing {
        id: id.to_string(),
    });

    rsx! {
        main {
            id: "listing",
            Title { prop: title }
            for listing in listings {
                Listing { prop: listing }
            }
        }
    }
}
