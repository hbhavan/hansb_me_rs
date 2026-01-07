use dioxus::prelude::*;

use crate::{
    components::{Listable, Listing},
    data::directory::Directory,
};

#[server]
pub async fn from_dir(dir: Directory) -> Result<Vec<Listing>, ServerFnError> {
    Ok(dir.listings())
}
