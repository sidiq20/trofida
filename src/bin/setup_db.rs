use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database: {}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let schema = fs::read_to_string("src/models/schema.sql")?;

    // Create a version without 'CREATE DATABASE' just in case we are already connected to it
    // or cannot switch.
    let commands: Vec<&str> = schema
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter(|s| !s.to_uppercase().starts_with("CREATE DATABASE"))
        .collect();

    for cmd in commands {
        println!("Executing: {:.50}...", cmd);
        if let Err(e) = pool.execute(cmd).await {
            println!("Error executing command: {}", e);
            // Don't exit, some might fail (e.g. table exists)
        }
    }

    println!("Schema setup complete.");
    Ok(())
}
