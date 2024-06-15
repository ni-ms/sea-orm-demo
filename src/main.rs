use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

const DATABASE_URL: &str = "sqlite::memory:";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    // Database agnostic, execute code based on matching database ( see docs )
    let db = &match db.get_database_backend() {
        DbBackend::Sqlite => db,
        _ => panic!("Unsupported database backend"),
    };


    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}