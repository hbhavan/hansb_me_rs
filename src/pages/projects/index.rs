use dioxus::prelude::*;

use crate::{
    components::{search::Search, *},
    pages::projects::content::{project_listings},
    data::*,
};

trait Filterable<T> {
    fn filt(&mut self, data: Self, item: T);
}

impl Filterable<Event<FormData>> for Vec<Listing> {
    fn filt(&mut self, listings: Vec<Listing>, item: Event<FormData>) {
        *self = listings
            .iter()
            .filter(|x| x.id().as_ref().contains(&item.value()))
            .map(|x| x.to_owned())
            .collect();
    }
}

#[component]
pub fn Projects() -> Element {
    let mut projects = use_signal(|| project_listings());


    rsx! {
        main { id: "projects",
            PageTitle { text: "Projects", size: TitleSize::Big }
            Search { on_search_change: move |e| filter(&mut projects.write(), project_listings(), e) }

            hr {}

            for (listing , skills) in projects.read().clone() {
                div { class: "listing",
                    Link { to: listing.route, {listing.title} }
                    for skill in skills {
                        Badge { badge_item: skill }
                    }
                }
            }
        }
    }
}

fn filter(projects: &mut Vec<(Listing, Vec<Skill>)>, listings: Vec<(Listing, Vec<Skill>)>, item: Event<FormData>) {
    *projects = listings
        .iter()
        .filter(|(x, _)| x.id().as_ref().contains(&item.value()))
        .map(|x| x.to_owned())
        .collect();
}
