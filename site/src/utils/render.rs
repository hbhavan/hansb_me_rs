use dioxus::prelude::*;

pub trait Render {
    fn render(&self) -> Element;
}

pub trait RenderMut {
    fn render(&'static mut self) -> Element;
}

pub mod markdown;
pub mod set_game;
