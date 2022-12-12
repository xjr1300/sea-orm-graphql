use std::env;

use dotenvy::dotenv;
use sea_orm::{ActiveValue, Database, DbErr, *};

mod entities;

use entities::{prelude::*, *};

async fn run(postgres_url: &str, database_name: &str) -> Result<(), DbErr> {
    let url = format!("{}/{}", postgres_url, database_name);
    let db = Database::connect(&url).await?;

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(String::from("Happy Bakery")),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let result = Bakery::insert(happy_bakery).exec(&db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(result.last_insert_id),
        name: ActiveValue::Set(String::from("Sad Bakery")),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(&db).await?;

    let john = chef::ActiveModel {
        name: ActiveValue::Set(String::from("John")),
        bakery_id: ActiveValue::Set(result.last_insert_id),
        ..Default::default()
    };
    Chef::insert(john).exec(&db).await?;

    let bakeries = Bakery::find().all(&db).await?;
    assert_eq!(bakeries.len(), 1);

    let sad_bakery = Bakery::find_by_id(1).one(&db).await?;
    assert_eq!(sad_bakery.unwrap().id, 1);

    let sad_bakery = Bakery::find()
        .filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(&db)
        .await?;
    assert_eq!(sad_bakery.unwrap().id, 1);

    let john = chef::ActiveModel {
        id: ActiveValue::Set(1),
        ..Default::default()
    };
    john.delete(&db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(1),
        ..Default::default()
    };
    sad_bakery.delete(&db).await?;

    let bakeries = Bakery::find().all(&db).await?;
    assert!(bakeries.is_empty());

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
