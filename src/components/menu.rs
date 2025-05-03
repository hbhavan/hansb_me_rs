use dioxus::prelude::*;

use crate::data::menu::{MenuItem, MenuMaker};

#[derive(Clone, Debug, PartialEq)]
pub struct MenuProp {
    pub menu_items: Vec<MenuItem>,
}

impl MenuProp {
    pub fn new(menu: &impl MenuMaker) -> Self {
        Self {
            menu_items: menu.to_menu(),
        }
    }
}

#[component]
pub fn Menu(prop: MenuProp) -> Element {
    rsx! {
        div {
            class: "menu",
            for item in prop.menu_items {
                a {
                    class: "menu-item",
                    href: item.path,
                    {item.title}
                }
                for sub_item in item.sub_items {
                    a {
                        class: "menu-sub-item",
                        href: sub_item.path,
                        {sub_item.title}
                    }
                }
            }
        }
    }
}
