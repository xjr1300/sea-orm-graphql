use std::env;

use dotenvy::dotenv;
use sea_orm::{Database, DbErr};

async fn run(postgres_url: &str) -> Result<(), DbErr> {
    let _db = Database::connect(postgres_url).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL not found in .env file");
    if let Err(e) = run(&postgres_url).await {
        panic!("{}", e);
    }
}
