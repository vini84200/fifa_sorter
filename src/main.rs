pub mod structures;
mod models;
mod reading;
mod knowledge;

use reading::{read_jogadores, read_rating, read_tags};
use knowledge::DB;

use anyhow::{Result, Ok};
use tracing;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut db = DB::new();
    initialize(&mut db).await?;
    let jogadores = db.search_jogador("Jo".to_string());
    print!("{:?}", jogadores);
    Ok(())
}

async fn initialize(db: &mut DB) -> Result<()> {
    let start = std::time::Instant::now();
    read_jogadores(db).await?;
    read_rating(db).await?;
    read_tags(db).await?;

    let elapsed = start.elapsed();
    println!("Time elapsed in initialization is: {:?}", elapsed);
    Ok(())
}