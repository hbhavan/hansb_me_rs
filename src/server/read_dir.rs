use std::fs::read_dir;

use dioxus::prelude::*;

use crate::data::directory::Directory;

#[server]
pub async fn from_dir(dir: Directory) -> Result<Vec<String>, ServerFnError> {
    let path = dir.get_path();

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

    Ok(result)
}
