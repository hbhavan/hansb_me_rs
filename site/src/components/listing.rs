use std::sync::Arc;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::navbar::Route;

pub trait Listable: Clone + PartialEq + 'static {
    fn to_listing(&self) -> Listing;

    fn display(&self) -> Element {
        rsx! {}
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
    id: Arc<str>,
    pub route: Route,
    pub title: String,
}

#[allow(dead_code)]
impl Listing {
    pub fn new(id: Arc<str>, title: String, route: Route) -> Self {
        Self { id, title, route }
    }

    pub fn id(&self) -> Arc<str> {
        self.id.clone()
    }
}

impl Listable for Listing {
    fn to_listing(&self) -> Listing {
        self.clone()
    }
}


#[component]
pub fn Listings<T: Listable>(listings: Vec<T>) -> Element {
    rsx! {
        for listing in listings {
            div { class: "listing",
                ListingLink { listing: listing.to_listing() }
                {listing.display()}
            }
        }
    }
}

#[component]
fn ListingLink(listing: Listing) -> Element {
    rsx! {
        Link {
            class: "listing-link",
            id: listing.id().as_ref().to_string(),
            to: listing.route,
            {listing.title}
        }
    }
}
