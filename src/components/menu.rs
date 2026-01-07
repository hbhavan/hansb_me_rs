use dioxus::prelude::*;

use crate::data::menu::MenuMaker;

#[component]
pub fn Menu<T: MenuMaker>(menu: T) -> Element {
    let menu_items = menu.to_menu();

    rsx! {
        if menu_items.len() > 0 {
            div {
                class: "menu",
                for item in menu_items {
                    a {
                        class: item.item_style(),
                        href: item.path,
                        {item.title.clone()}
                    }
                }
            }
        }
    }
}
