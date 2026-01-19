use dioxus::prelude::*;

pub trait Render {
    fn render(&self) -> Element;
}

pub mod markdown;
