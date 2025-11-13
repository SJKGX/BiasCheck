use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};

use crate::{config::env::ENVS, db::sea::migrate};

mod config;
mod db;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
    }
}

async fn run() -> Result<()> {
    let db: DatabaseConnection = Database::connect(&ENVS.database_url).await?;

    migrate(&db).await?;

    Ok(())
}
