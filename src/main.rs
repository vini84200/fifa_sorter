use anyhow::{Ok, Result};
use tracing;

use knowledge::DB;
use reading::{read_jogadores, read_rating, read_tags};

pub mod structures;
mod models;
mod reading;
mod knowledge;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut db = DB::new();
    // initialize(&mut db).await?;
    // let jogadores = db.search_jogador("Jo".to_string());
    // print!("{:?}", jogadores);

    let query = "top10 'ST'";
    let query = parser::parse_query(query);
    println!("{:?}", query);
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