use anyhow::{Ok, Result};

mod knowledge;
mod models;
mod parser;
mod reading;
pub mod structures;

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
