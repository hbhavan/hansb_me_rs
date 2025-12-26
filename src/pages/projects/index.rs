use dioxus::prelude::*;

use crate::components::{search::*, title::*};

#[component]
pub fn Projects() -> Element {
    let title = TitleProp::new(TitleSize::Big, "Projects");

    rsx! {
        main {
            id: "projects",
            Title { prop: title }
            Search {  }
        }
    }
}
