#![warn(clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
#![deny(clippy::missing_panics_doc)]
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
