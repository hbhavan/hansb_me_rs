use crate::{components::{
    section::{Section, SectionProp},
    title::*,
}, data::key_handlers::KeyEvents};
use dioxus::{prelude::*, logger::tracing::info};

#[component]
pub fn Home() -> Element {
    let title = TitleProp::new(TitleSize::Big, "Hans<B>.me");
    let subTitle = TitleProp::new(TitleSize::Medium, "Hans Bhavan");

    let section = SectionProp::new(section_text());

    let mut key_events = KeyEvents::new();
    let mut latest_key_pressed = String::from("");

    key_events.register(move |e: &Event<KeyboardData>| {
        use dioxus::events::Key::Character;
        info!("Key Pressed");
        info!("Latest pressed was: {latest_key_pressed}");

        if let Character(k) = e.key() {
            info!("{k}");
        }

        latest_key_pressed = e.key().to_string();
    });

    rsx! {
        div {
            tabindex: 0,
            class: "key-listener",
            onkeydown: move |e| key_events.act(&e),
            main {
                id: "home",
                Title { prop: title }
                Title { prop: subTitle }

                Section { prop: section }
            }
        }
    }
}

fn section_text<'a>() -> &'a str {
    "Full-Stack Software Development"
}
