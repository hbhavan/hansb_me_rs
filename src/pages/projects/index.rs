use dioxus::prelude::*;

use crate::{
    components::{
        listing::{Listing, ListingProp},
        navbar::Route,
        search::*,
        title::*,
    },
    pages::projects::content::project_listings,
};

#[component]
pub fn Projects() -> Element {
    let projects = use_signal(move || project_listings());

    rsx! {
        main {
            id: "projects",
            PageTitle { text: "Projects", size: TitleSize::Big }
            Search {  }

            for (id, title, _) in projects() {
                Link {
                    class: "listing",
                    to: Route::ProjectContent { id: id },
                    {title}
                }
            }
        }
    }
}
