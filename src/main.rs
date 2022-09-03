use anyhow::{Ok, Result};
use tracing;

use knowledge::DB;
use reading::{read_jogadores, read_rating, read_tags};
use crate::parser::Query;

pub mod structures;
mod models;
mod reading;
mod knowledge;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut db = DB::new();
    initialize(&mut db).await?;

    let query = "player jon".to_string();
    let query = Query::try_from(query);
    println!("{:?}", query);
    let res = db.run_query(query?);
    println!("{:?}", res);
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