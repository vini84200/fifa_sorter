use csv_async::AsyncDeserializer;
use anyhow::{Result, anyhow};
use tokio::fs::File;
use crate::structures::hash_table::HashTable;
use serde::Deserialize;
use tokio_stream::{self as stream, StreamExt};

#[derive(Debug, Clone, Default, Deserialize)]
struct Jogador {
    sofifa_id: u32,
    name: String,
    player_positions: String
}

#[derive(Debug, Clone, Default)]
pub struct JogadorComRating {
    pub name: String,
    pub player_positions: String,
    pub rating: f32,
    rating_count: u32,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Rating {
    pub(crate) user_id: u32,
    pub(crate) sofifa_id: u32,
    pub(crate) rating: f32,
}

#[derive(Debug, Clone, Default)]
pub struct User {
    pub(crate) ratings: Vec<Rating>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Tag {
    pub(crate) sofifa_id: u32,
    pub(crate) tag: String,
}

#[allow(dead_code)]
pub async fn read_tags(jogadores: &mut HashTable<u32, JogadorComRating>, tags: &mut HashTable<String, Vec<u32>>) -> Result<(), anyhow::Error> {
    // let mut tag_reader = Reader::from_path("data/tags.csv")?;
    let mut tag_reader = AsyncDeserializer::from_reader(
            File::open("data/tags.csv").await?
        );
    let mut deserialized_tags = tag_reader.deserialize::<Tag>();
    while let Some(tag) = deserialized_tags.next().await {
        if let Ok(tag) = tag {
            if let Some(jogador) = jogadores.get_mut(&tag.sofifa_id) {
                //Has tag?
                if !jogador.tags.contains(&tag.tag) {
                    jogador.tags.push(tag.tag.clone());
                }
                jogador.tags.push(tag.tag.clone());
            }
            if let Some(jogadores) = tags.get_mut(&tag.tag) {
                if !jogadores.contains(&tag.sofifa_id) {
                    jogadores.push(tag.sofifa_id);
                }
            } else {
                tags.insert(&tag.tag, vec![tag.sofifa_id]).unwrap();
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn read_rating(users: &mut HashTable<u32, User>, jogadores: &mut HashTable<u32, JogadorComRating>) -> Result<(), anyhow::Error> {
    // let mut count = 0;
    // let mut reader = Reader::from_path("data/rating.csv")?;
    let mut reader = AsyncDeserializer::from_reader(
            File::open("data/rating.csv").await?
        );
    let deserialized_ratings: Result<Vec<Rating>, _> = reader.deserialize::<Rating>()
        .collect()
        .await;
    if let Ok(ratings) = deserialized_ratings {

        ratings.into_iter().try_for_each(|rating| -> Result<(), anyhow::Error> {
            let record: Rating = rating;
            let id = record.sofifa_id;
            let score = record.rating;
            if let Some(user) = users.get_mut(&record.user_id ) {
                user.ratings.push(record);
            } else {
                users.insert(&record.user_id, User {
                    ratings: vec![record.clone()]
                })?;
            }
            if let Some(jogador) = jogadores.get_mut(&id) {
                jogador.rating = ((jogador.rating * jogador.rating_count as f32) + score) / (jogador.rating_count as f32 + 1.0);
                jogador.rating_count += 1;
            } else {
                Err(anyhow!("Jogador não encontrado"))?;
            }

            Ok(())
        })?;
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn read_jogadores(jogadores: &mut HashTable<u32, JogadorComRating>) -> Result<(), anyhow::Error> {
    // let mut reader = Reader::from_path("data/players.csv")?;
    let mut reader = AsyncDeserializer::from_reader(
            File::open("data/players.csv").await?
        );
    let deserialized_jogadores: Result<Vec<Jogador>, _> = reader.deserialize::<Jogador>().collect().await;

    deserialized_jogadores.unwrap().into_iter().try_for_each(|j| -> Result<(), anyhow::Error> {
        let record: Jogador = j;
        jogadores.insert(&record.sofifa_id, JogadorComRating {
            name: record.name,
            player_positions: record.player_positions,
            rating: 0.0,
            rating_count: 0,
            tags: Vec::new(),
        })?;
        Ok(())
    })?;
    Ok(())
}