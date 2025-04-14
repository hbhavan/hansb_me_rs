use std::fs::{read_dir, DirEntry};

use dioxus::prelude::*;

use super::navbar::Route;

#[derive(Clone, Debug, PartialEq)]
pub struct ListingProp {
    pub route: Route,
    pub title: String,
    pub path: String,
}

pub enum Directory {
    Markdown,
}

impl ListingProp {
    pub fn new(title: &str, path: &str, route: Route) -> Self {
        Self {
            title: title.to_string(),
            path: path.to_lowercase().to_string(),
            route,
        }
    }

    pub fn from_entry(entry: DirEntry, route_fn: fn(&str) -> Route) -> Self {
        let name = match entry.path().file_name().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => String::from(""),
        };

        let route = route_fn(&name);

        Self {
            title: name.to_uppercase(),
            path: name.to_lowercase(),
            route,
        }
    }

    pub fn from_dir(dir: Directory, route_fn: fn(&str) -> Route) -> Vec<ListingProp> {
        let path = format!("./store/{}", dir.get_name());

        match read_dir(path) {
            Ok(d) => d
                .into_iter()
                .map(|e| match e {
                    Ok(e) => ListingProp::from_entry(e, route_fn),
                    Err(_) => ListingProp::new("", "", Route::Home {}),
                })
                .collect::<Vec<ListingProp>>(),
            Err(_) => Vec::new(),
        }
    }
}

impl Directory {
    pub fn get_name(&self) -> &str {
        use Directory::*;

        match self {
            Markdown => "markdown",
        }
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
