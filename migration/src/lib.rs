// migration/src/lib.rs

pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20220101_000001_create_bakery_table;
mod m20220101_000002_create_chef_table;
// Run DATABASE_URL="sqlite::memory:" sea-orm-cli migrate refresh
/*
Generate the entity files for the migrations:
sea-orm-cli generate entity \
    -u sqlite::memory: \
    -o src/entities
 */
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20220101_000001_create_bakery_table::Migration),
            Box::new(m20220101_000002_create_chef_table::Migration),
        ]
    }
}
