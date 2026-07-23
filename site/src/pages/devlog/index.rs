use dioxus::prelude::*;

use crate::components::*;
use crate::layout::Route;

#[component]
pub fn DevLog() -> Element {
    rsx! {
        div { DevLogListings {} }
    }
}

#[component]
fn DevLogListings() -> Element {
    let devlog_listings = use_resource(get_devlog_listings);

    let curr = match &* devlog_listings.read() {
        Some(Ok(l)) => l.clone(),
        _ => vec![]
    };

    rsx! {
        for lst in curr.iter()
            .map(|x| Listing::new(
                x.0.to_string(),
                x.1.clone(),
                Route::DevLogContent {
                    devlog_id: x.0,
                },
            ))
        {
            li {
                ListingLink { listing: lst }
            }
        }
    }
}

#[get("/get_devlog_listings")]
async fn get_devlog_listings() -> Result<Vec<(i32, String)>, ServerFnError> {
    use services::api::devlog;

    let listings = devlog::get_devlog_listings_all().await
        .map_err(|_| ServerFnError::new("Db error"));

    listings
}
