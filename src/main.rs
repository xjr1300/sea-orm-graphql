use std::env;

use dotenvy::dotenv;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

async fn run(postgres_url: &str, database_name: &str) -> Result<(), DbErr> {
    let url = format!("{}/{}", postgres_url, database_name);
    let _db = Database::connect(&url).await?;
    // // Dockerを使用すする場合、POSTGRES_DBを使用しなくても、ユーザー名のデータベースを作成
    // // するためコメント・アウト
    // let _db = &match db.get_database_backend() {
    //     DbBackend::Postgres => {
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("DROP DATABASE IF EXISTS \"{}\";", database_name),
    //         ))
    //         .await?;
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("CREATE DATABASE \"{}\";", database_name),
    //         ))
    //         .await?;
    //         let url = format!("{}/{}", postgres_url, database_name);

    //         Database::connect(&url).await?
    //     }
    //     DbBackend::MySql => {
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("CREATE DATABASE IF NOT EXISTS `{}`;", database_name),
    //         ))
    //         .await?;
    //         let url = format!("{}/{}", postgres_url, database_name);

    //         Database::connect(&url).await?
    //     }
    //     DbBackend::Sqlite => db,
    // };

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
