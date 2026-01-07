use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::components::{navbar::Route, Listable, Listing};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Directory {
    Markdown,
}

#[allow(dead_code)]
impl Directory {
    pub fn get_name(&self) -> &str {
        use Directory::*;

        match self {
            Markdown => "markdown",
        }
    }

    pub fn get_path(&self) -> String {
        format!("./store/{}", self.get_name())
    }
}

impl Listable for Directory {
    fn listings(&self) -> Vec<Listing> {
        use std::fs::*;
        let path = self.get_path();

        let result = match read_dir(path) {
            Ok(d) => d
                .into_iter()
                .map(|ent| match ent {
                    Ok(e) => e
                        .path()
                        .file_name()
                        .and_then(|s| s.to_str())
                        .map(|s| String::from(s))
                        .unwrap_or(String::from("")),
                    Err(_) => String::from(""),
                })
                .collect::<Vec<String>>(),
            Err(_) => Vec::new(),
        };
        let lst = from_dir(result, |r| Route::DevLogListing { id: r.to_string() });

        return lst;
    }
}

fn from_dir(dirs: Vec<String>, route_fn: fn(&str) -> Route) -> Vec<Listing> {
    dirs.iter()
        .filter(|ent| ent.len() > 0)
        .map(|ent| Listing::new(ent.as_str().into(), ent.to_uppercase(), route_fn(&ent)))
        .collect::<Vec<Listing>>()
}
