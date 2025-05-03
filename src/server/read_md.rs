use std::{char, fs::File, io::Read, path::PathBuf};

use dioxus::{logger::tracing::info, prelude::*};
#[server]
pub async fn get_md_file(path: PathBuf) -> Result<String, ServerFnError> {
    let mut buf = String::from("");
    info!("Path: {:?}", path);

    match File::open(path).and_then(|mut f| f.read_to_string(&mut buf)) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e.into()),
    }
}
