mod hash_table;


use crate::hash_table::HashTable;
use csv::Reader;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
struct Jogador {
    sofifa_id: u32,
    name: String,
    player_positions: String
}

#[derive(Debug, Clone, Default)]
struct JogadorComRating {
    sofifa_id: u32,
    name: String,
    player_positions: String,
    rating: f32,
    rating_count: u32,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Rating {
    user_id: u32,
    sofifa_id: u32,
    rating: f32,
}

#[derive(Debug, Clone, Default)]
struct User {
    user_id: u32,
    ratings: Vec<Rating>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Tag {
    user_id: u32,
    sofifa_id: u32,
    tag: String,
}


fn main() -> Result<()>{

    // Timer
    let start = std::time::Instant::now();
    let start_total = std::time::Instant::now();

    let mut jogadores = HashTable::new(22807);
    let mut users : HashTable<u32, User> = HashTable::new(28800001);
    let mut tags : HashTable<String, Vec<u32>> = HashTable::new(438001);

    let mut reader = Reader::from_path("data/players.csv")?;
    for result in reader.deserialize() {
        let record: Jogador = result?;
        jogadores.insert(&record.sofifa_id, JogadorComRating {
            sofifa_id: record.sofifa_id,
            name: record.name,
            player_positions: record.player_positions,
            rating: 0.0,
            rating_count: 0,
            tags: Vec::new(),
        })?;
    }

    let ellapsed = start.elapsed();
    let start = std::time::Instant::now();
    println!("Jogadores: {:?}", ellapsed);

    let mut count = 0;


    let mut reader = Reader::from_path("data/rating.csv")?;
    for result in reader.deserialize() {
        count += 1;
        let record: Rating = result?;
        let id = record.sofifa_id;
        let score = record.rating;
        if let Some(user) = users.get_mut(&record.user_id ) {
            user.ratings.push(record);
        } else {
            users.insert(&record.user_id, User {
                user_id: record.user_id,
                ratings: vec![record.clone()]
            })?;
        }
        if let Some(jogador) = jogadores.get_mut(&id) {
            jogador.rating = ((jogador.rating * jogador.rating_count as f32) + score) / (jogador.rating_count as f32 + 1.0);
            jogador.rating_count += 1;
        }
        if count % 1000000 == 0 {
            println!("{}", count);
        }
        // println!("{}", id);

    }



    let ellapsed = start.elapsed();
    let start = std::time::Instant::now();
    println!("Ratings: {:?}", ellapsed);


    let mut tag_reader = Reader::from_path("data/tags.csv")?;
    for tag in tag_reader.deserialize() {
        let tag: Tag = tag?;
        if let Some(jogador) = jogadores.get_mut(&tag.sofifa_id) {
            jogador.tags.push(tag.tag.clone());
        }
        if let Ok(user_tags) = tags.get_mut_or_default(&tag.tag) {
            user_tags.push(tag.sofifa_id);
        }
    }

    let ellapsed = start.elapsed();
    // let start = std::time::Instant::now();
    println!("Tags: {:?}", ellapsed);
    let ellapsed = start_total.elapsed();
    // let start = std::time::Instant::now();
    println!("Total: {:?}", ellapsed);
    // println!("{:?}", jogadores);
    println!("{:?}", jogadores.get(&210212));
    for j in tags.get(&"Brazil".to_string()).unwrap() {
        // println!("{}", jogadores.get(&j).unwrap().name);
    }
    // println!("{:?}", );


    Ok(())
}