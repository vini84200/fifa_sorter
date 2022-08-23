mod hash_table;

use std::ptr::read;

use crate::hash_table::HashTable;
use csv::Reader;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
struct Jogador {
    sofifa_id: u32,
    name: String,
    player_positions: String,
}

#[derive(Debug, Clone, Default)]
struct JogadorComRating {
    sofifa_id: u32,
    name: String,
    player_positions: String,
    rating: f32,
    rating_count: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Rating {
    user_id: u32,
    sofifa_id: u32,
    rating: f32,
}

#[derive(Debug, Clone, Default)]
struct User {
    ratings: Vec<Rating>,
}


fn main() -> Result<()>{

    // Timer
    let start = std::time::Instant::now();

    let mut jogadores = HashTable::new(10000);
    let mut users : HashTable<u32, User> = HashTable::new(1000000);


    let mut reader = Reader::from_path("data/players.csv")?;
    for result in reader.deserialize() {
        let record: Jogador = result?;
        jogadores.insert(&record.sofifa_id, JogadorComRating {
            sofifa_id: record.sofifa_id,
            name: record.name,
            player_positions: record.player_positions,
            rating: 0.0,
            rating_count: 0,
        })?;
    }

    let mut count = 0;

    let mut reader = Reader::from_path("data/rating.csv")?;
    for result in reader.deserialize() {
        count += 1;
        let record: Rating = result?;
        let id = record.sofifa_id;
        let score = record.rating;
        if let Ok(user)  = users.get_mut_or_default(&record.user_id) {
            user.ratings.push(record);
        }
        if let Some(jogador) = jogadores.get_mut(&id) {
            jogador.rating = ((jogador.rating * jogador.rating_count as f32) + score) / (jogador.rating_count as f32 + 1.0);
            jogador.rating_count += 1;
        }
        if count % 100000 == 0 {
            println!("{}", count);
        }
        // println!("{}", id);

    }

    let ellapsed = start.elapsed();
    println!("{:?}", ellapsed);

    // println!("{:?}", jogadores);
    println!("{:?}", jogadores.get(&210212));
    

    Ok(())
}