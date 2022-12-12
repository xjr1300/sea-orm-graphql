use std::env;

use dotenvy::dotenv;
use sea_orm::{ActiveValue, Database, DbErr, *};

mod entities;

use entities::{prelude::*, *};

async fn run(postgres_url: &str, database_name: &str) -> Result<(), DbErr> {
    let url = format!("{}/{}", postgres_url, database_name);
    let db = Database::connect(&url).await?;

    basic_crud_operations(&db).await?;
    relationship_select(db).await?;

    Ok(())
}

async fn basic_crud_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(String::from("Happy Bakery")),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_result = Bakery::insert(happy_bakery).exec(db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(bakery_result.last_insert_id),
        name: ActiveValue::Set(String::from("Sad Bakery")),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(db).await?;

    let john = chef::ActiveModel {
        name: ActiveValue::Set(String::from("John")),
        bakery_id: ActiveValue::Set(bakery_result.last_insert_id),
        ..Default::default()
    };
    let chef_result = Chef::insert(john).exec(db).await?;

    let bakeries = Bakery::find().all(db).await?;
    assert_eq!(bakeries.len(), 1);

    let sad_bakery = Bakery::find_by_id(bakery_result.last_insert_id)
        .one(db)
        .await?;
    assert_eq!(sad_bakery.unwrap().id, bakery_result.last_insert_id);

    let sad_bakery = Bakery::find()
        .filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(db)
        .await?;
    assert_eq!(sad_bakery.unwrap().id, bakery_result.last_insert_id);

    let john = chef::ActiveModel {
        id: ActiveValue::Set(chef_result.last_insert_id),
        ..Default::default()
    };
    john.delete(db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(bakery_result.last_insert_id),
        ..Default::default()
    };
    sad_bakery.delete(db).await?;

    let bakeries = Bakery::find().all(db).await?;
    assert!(bakeries.is_empty());

    Ok(())
}

async fn relationship_select(db: DatabaseConnection) -> Result<(), DbErr> {
    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set(String::from("La Boulangerie")),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_result = Bakery::insert(la_boulangerie).exec(&db).await?;
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_string()),
            bakery_id: ActiveValue::Set(bakery_result.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(&db).await?;
    }
    let la_boulangerie = Bakery::find_by_id(bakery_result.last_insert_id)
        .one(&db)
        .await?
        .unwrap();
    let chefs = la_boulangerie.find_related(Chef).all(&db).await?;
    let mut chef_names: Vec<String> = chefs.into_iter().map(|c| c.name).collect();
    chef_names.sort_unstable();
    assert_eq!(
        chef_names,
        vec!["Charles", "Frederic", "Jolie", "Madeleine"]
    );
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
