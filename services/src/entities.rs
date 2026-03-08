use sea_orm::{DatabaseConnection, DbErr};
use std::env;

pub mod devlog;


pub async fn apply_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    println!("applying migrations");
    
    let reg = db.get_schema_registry(module_path!().split("::").next().unwrap());
    reg.apply(db).await?;

    Ok(())
}

pub async fn get_db() -> Result<DatabaseConnection, DbErr>  {
    println!("Connecting to db");

    let conn_str = env::var("DATABASE_URL")
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    let db = &sea_orm::Database::connect(conn_str).await?;

    Ok(db.clone())
}

