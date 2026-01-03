use dioxus::prelude::*;

#[cfg(feature = "server")]
pub async fn conn() -> Result<(), sea_orm::DbErr> {
    info!("Db connect");
    let db = sea_orm::Database::connect("postgres://hans:bhavan@localhost/database").await?;

    return Ok(());
}
