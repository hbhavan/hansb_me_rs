use sea_orm::DbErr;

pub mod devlog;

pub async fn db_conn() -> Result<(), DbErr> {
    println!("Connecting to db");
    let (user, pass, host, db) = ("hans", "6156", "localhost", "hb_me");
    let conn = format!("postgres://{user}:{pass}@{host}/{db}");

    let db = &sea_orm::Database::connect(conn).await?;

    let reg = db.get_schema_registry(module_path!().split("::").next().unwrap());
    reg.apply(db).await?;

    Ok(())
}

