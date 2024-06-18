use futures::executor::block_on;
use sea_orm::*;

mod entities;

use entities::{prelude::*, *};


const DATABASE_URL: &str = "sqlite://db.sqlite?mode=rwc";

const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    // Database agnostic, execute code based on matching database ( see docs )
    let db = &match db.get_database_backend() {
        DbBackend::Sqlite => db,
        _ => panic!("Unsupported database backend"),
    };

    //CRUD OPERATIONS
    //CREATE

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(db).await?;
    let john = chef::ActiveModel {
        name: ActiveValue::Set("John".to_owned()),
        bakery_id: ActiveValue::Set(res.last_insert_id), // a foreign key
        ..Default::default()
    };
    Chef::insert(john).exec(db).await?;

    //READ
    // Finding all is built-in
    let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
    assert_eq!(bakeries.len(), 1);

    // Finding by id is built-in
    let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(db).await?;
    assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

    // Finding by arbitrary column with `filter()`
    let sad_bakery: Option<bakery::Model> = Bakery::find()
        .filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(db)
        .await?;
    assert_eq!(sad_bakery.unwrap().id, 1);

    // DELETE
    let john = chef::ActiveModel {
        id: ActiveValue::Set(1), // The primary key must be set
        ..Default::default()
    };
    john.delete(db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(1), // The primary key must be set
        ..Default::default()
    };
    sad_bakery.delete(db).await?;

    let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
    assert!(bakeries.is_empty());


    // Performing Queries across different entities
    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;

    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}