use crate::components::{
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
        main {
            id: "home",
            Title { prop: title }
            Title { prop: subTitle }

            Section { prop: section }
        }
    }
}

fn section_text() -> String {
    "Full-Stack Software Development".into()
}
