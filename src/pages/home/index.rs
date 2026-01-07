use crate::{components::*, data::key_handlers::KeyEvents};
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Home() -> Element {
    let mut key_events = KeyEvents::new();
    let mut latest_key_pressed = String::from("");

    key_events.register(move |e: &Event<KeyboardData>| {
        info!("Key Pressed");
        info!("Latest pressed was: {latest_key_pressed}");

        if let Key::Character(k) = e.key() {
            info!("{k}");
        }

        latest_key_pressed = e.key().to_string();
    });

    rsx! {
        div {
            tabindex: 0,
            class: "key-listener",
            onkeydown: move |e| key_events.act(&e),
            main { id: "home",
                PageTitle { text: "Hans<B>.me", size: TitleSize::Big }
                PageTitle { text: "Hans Bhavan", size: TitleSize::Medium }

                Section { text: "Full-Stack Software Development" }
            }
        }
    }
}
