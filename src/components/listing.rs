use std::sync::Arc;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::navbar::Route;

pub trait Listable: Clone + PartialEq + 'static {
    fn listings(&self) -> Vec<Listing>;
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

impl Listable for Vec<Listing> {
    fn listings(&self) -> Vec<Listing> {
        self.clone()
    }
}


#[component]
pub fn Listings<T: Listable>(listable: T, children: Element) -> Element {
    rsx! {
        for listing in listable.listings() {
            Link {
                class: "listing",
                id: listing.id().as_ref().to_string(),
                to: listing.route,
                {listing.title}
            }
            {children.clone()}
        }
    }
}
