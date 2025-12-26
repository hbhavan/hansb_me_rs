use crate::components::{
    key_grid::KeyGrid,
    section::{Section, SectionProp},
    title::*,
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let title = TitleProp::new(TitleSize::Big, "Hans<B>.me");
    let subTitle = TitleProp::new(TitleSize::Medium, "Hans Bhavan");

    let section = SectionProp::new(&section_text());

    rsx! {
        div {
            main {
                id: "home",
                Title { prop: title }
                Title { prop: subTitle }

                Section { prop: section }

                KeyGrid { width: 5, height: 7 }
            }
        }
    }
}

fn section_text() -> String {
    "Full-Stack Software Development".into()
}
