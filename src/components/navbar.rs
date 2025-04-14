use dioxus::prelude::*;

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
    Home { },

    #[route("/devlog")]
    DevLog { },
    #[route("/devlog/listing/:id")]
    DevLogListing { id: String },

    #[route("/projects")]
    Projects { },

    #[route("/about")]
    About { },

    #[route("/error")]
    ErrorPage { }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            },
            Link {
                to: Route::DevLog {},
                "Dev Log"
            },
            Link {
                to: Route::Projects {},
                "Projects"
            },
            Link {
                to: Route::About {},
                "About"
            },
        }

        Outlet::<Route> {}
    }
}
