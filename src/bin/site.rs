extern crate hansb_me_3;
use dotenv::dotenv;
use services::entities::{apply_migrations, get_db};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Running site");
}
