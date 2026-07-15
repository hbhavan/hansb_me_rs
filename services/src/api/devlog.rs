use core::time;

use sea_orm::*;

use crate::entities::{devlog, get_db};

pub async fn get_devlog_listings_all() -> Result<Vec<(i32, String)>, DbErr> {
    let db = get_db().await?;

    let query = devlog::Entity::find()
        .order_by_desc(devlog::Column::CreatedDate)
        .all(&db)
        .await?;

    let listings = query
        .iter()
        .map(|x| (x.id, x.title.clone()))
        .collect();

    let delay = time::Duration::from_secs(1);
    std::thread::sleep(delay);

    Ok(listings)
}

pub async fn get_devlog_by_id(id: i32) -> Result<String, DbErr> {
    let db = get_db().await?;

    let query = devlog::Entity::find_by_id(id)
        .one(&db)
        .await?;

    let markdown = query
        .map(|x| x.markdown)
        .unwrap_or("### Devlog not found".into());

    Ok(markdown)
}
