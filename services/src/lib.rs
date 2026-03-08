use sea_orm::prelude::*;

pub mod api;
pub mod entities;

pub async fn check_conn(db: DatabaseConnection) -> Result<(), DbErr> {
    assert!(db.ping().await.is_ok());

    db.clone().close().await?;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));

    Ok(())
}

#[cfg(test)]
mod tests {
}
