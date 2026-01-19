use dioxus::prelude::*;

use crate::{
    components::*, 
    data::*,
    pages::projects::content::{project_listings}
};

#[component]
pub fn Projects() -> Element {
    let mut projects = use_signal(|| project_listings());


    rsx! {
        main { id: "projects",
            PageTitle { text: "Projects", size: TitleSize::Big }
            Search { on_search_change: move |e| filter(&mut projects.write(), project_listings(), e) }

            hr {}

            Listings { listings: projects.read().clone() }
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
