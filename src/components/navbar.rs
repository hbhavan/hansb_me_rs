use dioxus::logger::tracing::info;
use dioxus::prelude::*;

use crate::data::KeyboardSelector;
use crate::pages::about::index::About;
use crate::pages::devlog::content::DevLogListing;
use crate::pages::devlog::index::DevLog;
use crate::pages::error::index::ErrorPage;
use crate::pages::home::index::Home;
use crate::pages::projects::index::Projects;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home,

    #[route("/devlog")]
    DevLog,
    #[route("/devlog/listing/:id")]
    DevLogListing { id: String },

    #[route("/projects")]
    Projects,

    #[route("/about")]
    About,

    #[route("/error")]
    ErrorPage 
}

#[derive(Clone)]
struct NavLink<'a> {
    pub route: Route,
    pub text: &'a str,
    pub key: char,
    pub selected: bool,
}

impl<'a> NavLink<'a> {
    pub fn new(route: Route, text: &'a str, key: char) -> Self {
        Self {
            route,
            text,
            key,
            selected: false,
        }
    }
}

impl<'a> KeyboardSelector for Vec<NavLink<'a>> {
    fn on_key_press(&mut self, evt: Event<KeyboardData>) {
        info!("Key pressed");

        if let Key::Character(k) = evt.key() {
            for q in self.iter_mut() {
                if q.key.to_string() == k.to_uppercase().as_str() {
                    q.selected = true;
                } else {
                    q.selected = false;
                }
            }
        }
    }
}

fn get_nav_links<'a>() -> Vec<NavLink<'a>> {
    let home = NavLink::new(Route::Home, "Home", 'H');
    let devlog = NavLink::new(Route::DevLog, "Dev Log", 'D');
    let projects = NavLink::new(Route::Projects, "Projects", 'P');
    let about = NavLink::new(Route::About, "About", 'A');

    vec![home, devlog, projects, about]
}

#[component]
fn Navbar() -> Element {
    let nav_links = get_nav_links();
    let mut n = use_signal(|| nav_links);

    rsx! {
        div {
            class: "keyboard-listener",
            tabindex: 0,
            onkeydown: move |e| n.write().on_key_press(e),
            nav {
                class: "nav-bar",
                for nav_link in n.read().clone() {
                    div {
                        class: "nav-link",
                        Link {
                            to: nav_link.route,
                            {nav_link.text}
                        }
                        if nav_link.selected {
                            span {
                                class: "selected",
                                {format!("[{}]", nav_link.key)}
                            }
                        } else {
                            span { {format!("[{}]", nav_link.key)} }
                        }
                    }
                }
            },
        }

        Outlet::<Route> {}
    }
}
