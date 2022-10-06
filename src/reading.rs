use anyhow::Result;
use csv::Reader;

use crate::knowledge::DB;
use crate::models::*;

#[allow(dead_code)]
pub fn read_tags(db: &mut DB) -> Result<(), anyhow::Error> {
    let mut tag_reader = Reader::from_path("data/tags.csv")?;

    tag_reader
        .deserialize()
        .try_for_each(|tag| -> Result<(), anyhow::Error> {
            let tag: Tag = tag?;
            db.insert_tag(&tag)?;
            Ok(())
        })?;

    Ok(())
}

#[allow(dead_code)]
pub fn read_rating(db: &mut DB) -> Result<(), anyhow::Error> {
    let mut reader = Reader::from_path("data/rating.csv")?;
    let mut count = 0;
    for result in reader.deserialize() {
        let rating: Rating = result?;
        db.insert_rating(&rating)?;
        count += 1;
        if count % 1_000_000 == 0 {
            // println!("{} ratings read", count);
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn read_jogadores(db: &mut DB) -> Result<(), anyhow::Error> {
    let mut reader = Reader::from_path("data/players.csv")?;
    reader
        .deserialize()
        .try_for_each(|record| -> Result<(), anyhow::Error> {
            let jogador: Jogador = record?;
            db.insert_jogador(&jogador)?;
            Ok(())
        })?;
    Ok(())
}

pub fn initialize(db: &mut DB) -> Result<()> {
    read_jogadores(db)?;
    read_rating(db)?;
    read_tags(db)?;

    db.finish_init();
    Ok(())
}
