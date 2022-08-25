mod hash_table;
mod reading;

use crate::hash_table::HashTable;
use anyhow::Result;



fn main() -> Result<()>{

    // Timer
    let start_total = std::time::Instant::now();

    let mut jogadores = HashTable::new(22807);
    let mut users : HashTable<u32, reading::User> = HashTable::new(28800001);
    let mut tags : HashTable<String, Vec<u32>> = HashTable::new(438001);

    reading::read_jogadores(&mut jogadores)?;
    reading::read_rating(users, &mut jogadores)?;
    reading::read_tags(&mut jogadores, &mut tags)?;


    let ellapsed = start_total.elapsed();
    println!("Total: {:?}", ellapsed);

    Ok(())
}