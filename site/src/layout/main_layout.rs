use dioxus::prelude::*;
use crate::components::{heading::{Heading, HeadingSize}, logo::Logo, navbar::NavBar};
use super::Route;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        Banner {}

        Content {}

        Footer {}
    }
}

#[component]
fn Banner() -> Element {
    rsx!{
        header {
            div { class: "banner", Logo {} }
            NavBar {}
        }
    }
}

#[component]
fn Content() -> Element {
    let route = use_route::<Route>();
    let title = route
        .title()
        .map(|text| rsx!{
            Heading { text, size: HeadingSize::Medium }
        });

    rsx! {
        main {
            {title}

            Outlet::<Route> {}
        }
    }
}


#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            div { class: "footer-container", Logo {} }
        }
    }
}
