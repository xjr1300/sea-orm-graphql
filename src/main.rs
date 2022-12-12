use std::env;

use dotenvy::dotenv;
use sea_orm::{ActiveValue, Database, DbErr, *};

mod entities;

use entities::{prelude::*, *};

async fn run(postgres_url: &str, database_name: &str) -> Result<(), DbErr> {
    let url = format!("{}/{}", postgres_url, database_name);
    let db = Database::connect(&url).await?;

    delete_records(&db).await?;

    basic_crud_operations(&db).await?;
    relationship_select(&db).await?;
    test_with_mock().await?;

    Ok(())
}

async fn delete_records(db: &DatabaseConnection) -> Result<(), DbErr> {
    chef::Entity::delete_many().exec(db).await?;
    bakery::Entity::delete_many().exec(db).await?;

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

async fn relationship_select(db: &DatabaseConnection) -> Result<(), DbErr> {
    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set(String::from("La Boulangerie")),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_result = Bakery::insert(la_boulangerie).exec(db).await?;

    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_string()),
            bakery_id: ActiveValue::Set(bakery_result.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }

    let la_boulangerie = Bakery::find_by_id(bakery_result.last_insert_id)
        .one(db)
        .await?
        .unwrap();
    let chefs = la_boulangerie.find_related(Chef).all(db).await?;
    let mut chef_names: Vec<String> = chefs.into_iter().map(|c| c.name).collect();
    chef_names.sort_unstable();
    assert_eq!(
        chef_names,
        vec!["Charles", "Frederic", "Jolie", "Madeleine"]
    );

    Ok(())
}

async fn test_with_mock() -> Result<(), DbErr> {
    let db = &MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            // 1つ目のクエリが予期する結果
            vec![bakery::Model {
                id: 1,
                name: String::from("Happy Bakery"),
                profit_margin: 0.0,
            }],
            // 2つ目のクエリが予期する結果
            vec![
                bakery::Model {
                    id: 1,
                    name: String::from("Happy Bakery"),
                    profit_margin: 0.0,
                },
                bakery::Model {
                    id: 2,
                    name: String::from("Sad Bakery"),
                    profit_margin: 100.0,
                },
                bakery::Model {
                    id: 3,
                    name: String::from("La Boulangerie"),
                    profit_margin: 17.89,
                },
            ],
        ])
        .append_query_results(vec![
            // 3つ目のクエリが予期する結果
            vec![
                chef::Model {
                    id: 1,
                    name: "Jolie".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 2,
                    name: "Charles".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 3,
                    name: "Madeleine".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 4,
                    name: "Frederic".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
            ],
        ])
        .into_connection();

    let happy_bakery = Bakery::find().one(db).await?.unwrap();
    assert_eq!(
        happy_bakery,
        bakery::Model {
            id: 1,
            name: String::from("Happy Bakery"),
            profit_margin: 0.0,
        }
    );

    let all_bakeries = Bakery::find().all(db).await?;
    assert_eq!(
        all_bakeries,
        vec![
            bakery::Model {
                id: 1,
                name: "Happy Bakery".to_owned(),
                profit_margin: 0.0,
            },
            bakery::Model {
                id: 2,
                name: "Sad Bakery".to_owned(),
                profit_margin: 100.0,
            },
            bakery::Model {
                id: 3,
                name: "La Boulangerie".to_owned(),
                profit_margin: 17.89,
            },
        ]
    );

    let la_boulangerie_chefs = Chef::find().all(db).await?;
    assert_eq!(
        la_boulangerie_chefs,
        vec![
            chef::Model {
                id: 1,
                name: "Jolie".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
            chef::Model {
                id: 2,
                name: "Charles".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
            chef::Model {
                id: 3,
                name: "Madeleine".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
            chef::Model {
                id: 4,
                name: "Frederic".to_owned(),
                contact_details: None,
                bakery_id: 3,
            },
        ]
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
