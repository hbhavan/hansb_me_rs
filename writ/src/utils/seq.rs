use std::{slice::Iter, sync::Arc, usize};

#[derive(Clone, Debug, PartialEq)]
pub struct Seq<T: Clone> {
    items: Arc<[T]>,
}

#[allow(dead_code)]
impl<T: Clone> Seq<T> {
    pub fn new() -> Self {
        Self {
            items: Arc::new([]),
        }
    }

    pub fn single(item: T) -> Self {
        Self {
            items: Arc::new([item]),
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self {
            items: Arc::from_iter(vec.into_boxed_slice()),
        }
    }

    pub fn from_slice(slc: &[T]) -> Self {
        Self {
            items: Arc::from_iter(slc.iter().map(|x| x.clone())),
        }
    }

    pub fn items(&self) -> Arc<[T]> {
        self.items.clone()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }

    pub fn append(&self, other: Seq<T>) -> Self {
        let items = vec![self.items.clone(), other.items.clone()].concat();

        Self {
            items: items.into(),
        }
    }

    pub fn append_item(&self, item: T) -> Self {
        let items = vec![self.items.clone(), Arc::new([item])].concat();

        Self {
            items: items.into(),
        }
    }

    pub fn prepend(&self, other: Seq<T>) -> Self {
        let items = vec![other.items.clone(), self.items.clone()].concat();

        Self {
            items: items.into(),
        }
    }

    pub fn prepend_item(&self, item: T) -> Self {
        let items = vec![Arc::new([item]), self.items.clone()].concat();

        Self {
            items: items.into(),
        }
    }

    pub fn insert_at(&self, item: T, n: usize) -> Self {
        let first = self.take(n);
        let last = self.skip(n);

        first.append_item(item).append(last)
    }

    pub fn map<TResult: Clone>(&self, map: fn(T) -> TResult) -> Seq<TResult> {
        let items = self.items().iter().map(move |x| map(x.clone())).collect();

        Seq { items }
    }

    pub fn take(&self, n: usize) -> Seq<T> {
        let items = self.items[..n].iter().map(|x| x.clone());

        Seq {
            items: Arc::from_iter(items),
        }
    }

    pub fn skip(&self, n: usize) -> Seq<T> {
        let items = self.items[n..].iter().map(|x| x.clone());

        Seq {
            items: Arc::from_iter(items),
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.items.iter().nth(0)
    }

    pub fn last(&self) -> Option<&T> {
        self.items.iter().nth(self.len())
    }

    pub fn get(&self, f: fn(&T) -> bool) -> Option<&T> {
        self.items.iter().find(|x| f(x))
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.items.to_vec()
    }

    pub fn to_slice(&self) -> &[T] {
        self.items.iter().as_slice()
    }
}

#[allow(dead_code)]
impl Seq<String> {
    pub fn to_string(&self) -> String {
        self.items
            .iter()
            .fold(String::from(""), |acc, s| format!("{}{}", acc, s))
    }
}

#[allow(dead_code)]
impl Seq<char> {
    pub fn to_string(&self) -> String {
        String::from_iter(self.items.iter().map(|c| *c))
    }

    pub fn to_char_seq(s: &String) -> Seq<char> {
        Seq::from_vec(s.chars().map(|c| c).collect::<Vec<char>>())
    }
}
