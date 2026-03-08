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
        div { class: "post-content",
            div {
                Search { on_search_change: move |e| filter(&mut projects.write(), project_listings(), e) }
            }
            article {
                Heading { text: "Projects", size: HeadingSize::Big }
                Listings { listings: projects.read().clone() }
            }
            div {}
        }
    }
}

fn filter(projects: &mut Vec<(Listing, Vec<Skill>)>, listings: Vec<(Listing, Vec<Skill>)>, item: Event<FormData>) {
    *projects = listings
        .iter()
        .filter(|(x, _)| x.title.contains(&item.value()))
        .map(|x| x.to_owned())
        .collect();
}
