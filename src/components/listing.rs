use dioxus::prelude::*;

use super::navbar::Route;

#[derive(Clone, Debug, PartialEq)]
pub struct ListingProp {
    pub route: Route,
    pub title: String,
    pub path: String,
}

impl ListingProp {
    pub fn new(title: &str, path: &str, route: Route) -> Self {
        Self {
            title: title.to_string(),
            path: path.to_lowercase().to_string(),
            route,
        }
    }

    pub fn from_dir(dirs: Vec<String>, route_fn: fn(&str) -> Route) -> Vec<ListingProp> {
        dirs.iter()
            .filter(|ent| ent.len() > 0)
            .map(|ent| ListingProp {
                title: ent.to_uppercase(),
                path: ent.to_lowercase(),
                route: route_fn(&ent),
            })
            .collect::<Vec<ListingProp>>()
    }
}

#[component]
pub fn Listing(prop: ListingProp) -> Element {
    rsx! {
        Link {
            class: "listing",
            to: prop.route,
            {prop.title}
        }
    }
}
