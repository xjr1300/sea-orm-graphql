use std::env;

use dotenvy::dotenv;
use sea_orm::{Database, DbErr};

async fn run(postgres_url: &str, database_name: &str) -> Result<(), DbErr> {
    let url = format!("{}/{}", postgres_url, database_name);
    let _db = Database::connect(&url).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL not found in .env file");
    let database_name =
        env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE not found in .env file");
    if let Err(e) = run(&postgres_url, &database_name).await {
        panic!("{}", e);
    }
}
