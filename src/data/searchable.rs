use std::fmt::Debug;

use dioxus::{core::Event, html::FormData};

pub trait Searchable: SearchItem + Clone + PartialEq + Debug + 'static {}

pub trait SearchItem {
    fn text(&self) -> &String;

    fn value(&self) -> String;
}

impl Searchable for String {}

impl SearchItem for String {
    fn text(&self) -> &String {
        self
    }

    fn value(&self) -> String {
        self.clone()
    }
}

pub fn filter<T: Searchable>(result: &mut Vec<T>, search_items: Vec<T>, search_value: &T) {
    *result = search_items
        .iter()
        .filter(|x| x.value().contains(&search_value.value()))
        .map(|x| x.to_owned())
        .collect();
}

pub fn filter_evt<T: Searchable>(result: &mut Vec<T>, search_items: Vec<T>, search_value: Event<FormData>) {
    *result = search_items
        .iter()
        .filter(|x| x.value().contains(&search_value.value()))
        .map(|x| x.to_owned())
        .collect();
}
