pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250114_103109_price_history;
mod m20250114_103705_notification_preferences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250114_103109_price_history::Migration),
            Box::new(m20250114_103705_notification_preferences::Migration),
        ]
    }
}
