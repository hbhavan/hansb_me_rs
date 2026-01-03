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
