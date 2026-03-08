use dioxus::prelude::*;

use crate::layout::Route;

#[derive(Clone)]
struct NavLink<'a> {
    pub route: Route,
    pub text: &'a str,
}

impl<'a> NavLink<'a> {
    pub fn new(route: Route, text: &'a str) -> Self {
        Self {
            route,
            text,
        }
    }
}

fn get_nav_links<'a>() -> Vec<NavLink<'a>> {
    let home = NavLink::new(Route::Home, "Home");
    let devlog = NavLink::new(Route::DevLog, "Dev Log");
    let projects = NavLink::new(Route::Projects, "Projects");
    let about = NavLink::new(Route::About, "About");

    vec![home, devlog, projects, about]
}

#[component]
pub fn NavBar() -> Element {
    let nav_links = get_nav_links();

    rsx! {
        nav { class: "nav-bar",
            for nav_link in nav_links {
                div { class: "nav-link",
                    Link { to: nav_link.route, {nav_link.text} }
                }
            }
        }
    }
}
