use dioxus::prelude::*;

pub trait BadgeItem: Clone + PartialEq + 'static {
    fn label(&self) -> String;
    fn icon(&self) -> Asset;
    fn style(&self) -> String {
        String::from("") 
    }
}

#[component]
pub fn Badge<T: BadgeItem>(badge_item: T) -> Element {
    rsx! {
        div { class: "badge", style: badge_item.style(),
            img { class: "badge-icon", src: badge_item.icon() }
            label { class: "badge-label", {badge_item.label()} }
        }
    }
}
