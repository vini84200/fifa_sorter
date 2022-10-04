use anyhow::{Ok, Result};

mod knowledge;
mod models;
mod parser;
mod reading;
pub mod structures;

#[cfg(feature = "terminal")]
mod terminal;

fn main() -> Result<()> {

    #[cfg(feature = "terminal")]
    terminal::main_loop();

    // Run if feature gui
    // TODO: Implement GUI
    Ok(())
}
