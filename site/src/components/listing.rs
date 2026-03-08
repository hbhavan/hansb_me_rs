use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::layout::Route;

pub trait Listable: Clone + PartialEq + 'static {
    fn to_listing(&self) -> Listing;

    fn display(&self) -> Element {
        rsx! {}
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
    id: i32,
    pub route: Route,
    pub title: String,
}

#[allow(dead_code)]
impl Listing {
    pub fn new(id: i32, title: String, route: Route) -> Self {
        Self { id, title, route }
    }

    pub fn id(&self) -> i32 {
        self.id
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
        Link { class: "listing-link", id: listing.id(), to: listing.route, {listing.title} }
    }
}
