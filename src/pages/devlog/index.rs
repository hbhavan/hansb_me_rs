use dioxus::prelude::*;

use crate::{components::*};

#[component]
pub fn DevLog() -> Element {
    let md_listings = use_server_future(move || get_devlog_listings())?;

    let listings = match md_listings() {
        Some(x) => match x {
            Ok(l) => l,
            Err(_) => vec![]
        },
        None => vec![]
    };

    let db_lst = use_server_future(move || get_devlog_listings_db())?;

    match db_lst() {
        Some(_) => info!("Got db"),
        None => info!("Didn't get db")
    };

    rsx! {
        main { id: "dev-log",
            PageTitle { text: "Dev Log", size: TitleSize::Big }
            Listings { listings }
        }
    }
}

#[get("/get_devlog_listings")]
async fn get_devlog_listings() -> Result<Vec<Listing>, ServerFnError> {
    use crate::{ data::directory::*};

    let directory = Directory::Markdown;
    let listings = directory.listings();

    Ok(listings)
}

#[get("/get_devlog_listings_db")]
async fn get_devlog_listings_db() -> Result<Vec<Listing>, ServerFnError> {
    // info!("Connecting to db");
    // let (user, pass, host) = ("postgres", "6156", "localhost");
    // let conn = format!("postgres://{user}:{pass}@{host}/postgres");
    //
    // let db = sea_orm::Database::connect(conn).await;
    //
    // match db {
    //     Ok(_) => info!("Connected"),
    //     Err(e) => info!("Error: {}", e)
    // };

    return Ok(vec![]);
}
