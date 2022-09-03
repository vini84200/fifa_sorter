use anyhow::{Ok, Result};

use knowledge::DB;
use reading::{read_jogadores, read_rating, read_tags};
use crate::parser::Query;

pub mod structures;
mod models;
mod reading;
mod knowledge;
mod parser;
mod simple_uses;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    simple_uses::main_loop().await;
    Ok(())
}
