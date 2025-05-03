use super::{markdown::Text, seq::Seq};

#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub title: String,
    pub path: String,
    pub sub_items: Vec<MenuItem>,
}

pub trait MenuMaker {
    fn to_menu(&self) -> Vec<MenuItem>;
}

impl MenuItem {
    pub fn new(title: &str, path: &str) -> Self {
        Self {
            title: title.to_string(),
            path: path.to_string(),
            sub_items: Vec::new(),
        }
    }

    pub fn sub_menu(title: &str, path: &str, sub_items: Vec<MenuItem>) -> Self {
        Self {
            title: title.to_string(),
            path: path.to_string(),
            sub_items,
        }
    }

    pub fn empty() -> Self {
        Self {
            title: String::from(""),
            path: String::from(""),
            sub_items: Vec::new(),
        }
    }

    pub fn from_markdown(text: &Seq<Text>) -> Self {
        let content = text
            .iter()
            .map(|t| t.get_text().to_string())
            .collect::<Vec<_>>();
        let title = content.join(" ");
        let path = format!("#{}", content.join("_").replace(" ", "_"));

        Self::new(&title, &path)
    }
}
