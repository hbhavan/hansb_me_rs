use crate::components::menu::Menu;

use super::{markdown::Text, seq::Seq};

#[derive(Clone, Debug, PartialEq)]
pub enum MenuItemType {
    Main,
    Sub,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub title: String,
    pub path: String,
    menu_item_type: MenuItemType,
}

pub trait MenuMaker {
    fn to_menu(&self) -> Vec<MenuItem>;
}

#[allow(dead_code)]
impl MenuItem {
    pub fn main_item(title: &str, path: &str) -> Self {
        Self::new(title, path, MenuItemType::Main)
    }

    pub fn sub_item(title: &str, path: &str) -> Self {
        Self::new(title, path, MenuItemType::Sub)
    }

    pub fn empty() -> Self {
        Self::new("", "", MenuItemType::Main)
    }

    pub fn from_markdown(size: usize, text: &Seq<Text>) -> Self {
        let content = text
            .iter()
            .map(|t| t.get_text().to_string())
            .collect::<Vec<_>>();
        let title = content.join(" ");
        let path = format!("#{}", content.join("_").replace(" ", "_"));
        let menu_item_type = match size {
            1 => MenuItemType::Main,
            2 => MenuItemType::Sub,
            _ => MenuItemType::Main,
        };

        Self::new(&title, &path, menu_item_type)
    }

    pub fn item_style(&self) -> String {
        match self.menu_item_type {
            MenuItemType::Main => String::from("menu-main"),
            MenuItemType::Sub => String::from("menu-sub"),
        }
    }

    fn new(title: &str, path: &str, menu_item_type: MenuItemType) -> Self {
        Self {
            title: title.to_string(),
            path: path.to_string(),
            menu_item_type,
        }
    }
}
