use std::path::PathBuf;

use dioxus::prelude::*;

#[server]
pub async fn get_md_file(path: PathBuf) -> Result<String, ServerFnError> {
    use dioxus::logger::tracing::info;
    use std::char;
    use std::fs::File;
    use std::io::Read;

    let mut buf = String::from("");
    info!("Path: {:?}", path);

    match File::open(path).and_then(|mut f| f.read_to_string(&mut buf)) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e.into()),
    }
}
