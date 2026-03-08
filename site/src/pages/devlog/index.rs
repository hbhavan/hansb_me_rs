use dioxus::prelude::*;

use crate::components::*;
use crate::layout::Route;

#[component]
pub fn DevLog() -> Element {
    let db_lst = use_server_future(move || get_devlog_listings())?;

    let listings = db_lst()
        .map(|x| match x {
            Ok(l) => l,
            Err(_) => vec![]
        })
        .unwrap_or(vec![])
        .iter()
        .map(|x| {
            Listing::new(
                x.0,
                x.1.clone(), 
                Route::DevLogListing { id: x.0 })
        })
        .collect();

    rsx! {
        main { id: "dev-log",
            Heading { text: "Dev Log", size: HeadingSize::Big }
            Listings { listings }
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
