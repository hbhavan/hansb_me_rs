use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "logo",
            span { class: "text-purple", "H" }
            span { class: "text-gold", "B" }
            span { class: "text-blue", ".me" }
        }
    }
}
