use anyhow::{Ok, Result};

use crate::parser::Query;

pub mod structures;
mod models;
mod reading;
mod knowledge;
mod parser;

#[cfg(feature = "terminal")]
mod terminal;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    #[cfg(feature = "terminal")]
    terminal::main_loop().await;

    // Run if feature gui
    // TODO: Implement GUI
    Ok(())
}
