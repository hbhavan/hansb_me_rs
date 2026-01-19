extern crate hansb_me_3;
use services::entities::db_conn;

#[tokio::main]
async fn main() {
    println!("Running site");

    let q = db_conn().await;

    match q {
        Ok(_) => println!("Db Connection established"),
        Err(e) => println!("{e}")
    };

}
