use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};

use crate::{db::sea::migrate, domain::env::ENVS};

mod db;
mod domain;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
    }
}

async fn run() -> Result<()> {
    let db: DatabaseConnection = Database::connect(&ENVS.database_url).await?;
    db.ping();
    migrate(&db).await?;

    Ok(())
}
