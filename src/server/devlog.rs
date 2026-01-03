#[cfg(feature = "server")]
use dioxus::prelude::*;

use crate::server::db;

pub mod listings;
pub mod post;

#[server]
pub async fn get_listings() -> Result<Vec<String>, ServerFnError> {
    info!("Getting listings");

    match db_conn().await {
        Ok(_) => {}
        Err(err) => return Err(ServerFnError::new(format!("Db connection failed - {err}"))),
    };

    info!("Db connection finished");

    return Ok(vec![]);
}

#[server]
async fn db_conn() -> Result<(), ServerFnError> {
    info!("AHHHHHHH");
    let z = db::conn().await;

    match z {
        Ok(_) => {
            info!("Connected to db!");
        }
        Err(e) => {
            error!("Could not connect to db - {e:?}");
        }
    }
    Ok(())
}
