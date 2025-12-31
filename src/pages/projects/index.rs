use dioxus::prelude::*;

use crate::components::{search::*, title::*};

#[component]
pub fn Projects() -> Element {
    rsx! {
        main {
            id: "projects",
            PageTitle { text: "Projects", size: TitleSize::Big }
            Search {  }
        }
    }
}
