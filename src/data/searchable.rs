use std::fmt::Debug;

pub trait Searchable: SearchItem + Clone + PartialEq + Debug + 'static {}

pub trait SearchItem {
    fn text(&self) -> &String;

    fn value(&self) -> &String;
}

impl Searchable for String {}

impl SearchItem for String {
    fn text(&self) -> &String {
        self
    }

    fn value(&self) -> &String {
        self
    }
}

pub fn filter<T: Searchable>(search_items: &Vec<T>, search_value: &T) -> Vec<T> {
    let result: Vec<T> = search_items
        .iter()
        .filter(|x| x.value() == search_value.value())
        .map(|x| x.clone())
        .collect();

    result
}
