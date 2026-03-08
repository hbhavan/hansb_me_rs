use dioxus::prelude::*;
use crate::components::{logo::Logo, navbar::NavBar};
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
    rsx! {
        main { Outlet::<Route> {} }
    }
}


#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            hr {}
            div { class: "footer-container", Logo {} }
        }
    }
}
