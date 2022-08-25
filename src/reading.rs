use csv::Reader;
    use anyhow::anyhow;
    use anyhow::Result;
    use crate::hash_table::HashTable;
    use serde::Deserialize;

    #[derive(Debug, Clone, Default, Deserialize)]
    pub(crate) struct Jogador {
        pub(crate) sofifa_id: u32,
        pub(crate) name: String,
        pub(crate) player_positions: String
    }

    #[derive(Debug, Clone, Default)]
    pub(crate) struct JogadorComRating {
        pub(crate) sofifa_id: u32,
        pub(crate) name: String,
        pub(crate) player_positions: String,
        pub(crate) rating: f32,
        pub(crate) rating_count: u32,
        pub(crate) tags: Vec<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub(crate) struct Rating {
        pub(crate) user_id: u32,
        pub(crate) sofifa_id: u32,
        pub(crate) rating: f32,
    }

    #[derive(Debug, Clone, Default)]
    pub(crate) struct User {
        pub(crate) user_id: u32,
        pub(crate) ratings: Vec<Rating>,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub(crate) struct Tag {
        pub(crate) user_id: u32,
        pub(crate) sofifa_id: u32,
        pub(crate) tag: String,
    }

    pub(crate) fn read_tags(jogadores: &mut HashTable<u32, JogadorComRating>, tags: &mut HashTable<String, Vec<u32>>) -> Result<(), anyhow::Error> {
        let mut tag_reader = Reader::from_path("data/tags.csv")?;
        Ok(for tag in tag_reader.deserialize() {
            let tag: Tag = tag?;
            if let Some(jogador) = jogadores.get_mut(&tag.sofifa_id) {
                jogador.tags.push(tag.tag.clone());
            }
            if let Ok(user_tags) = tags.get_mut_or_default(&tag.tag) {
                user_tags.push(tag.sofifa_id);
            }
        })
    }

    pub(crate) fn read_rating(mut users: HashTable<u32, User>, jogadores: &mut HashTable<u32, JogadorComRating>) -> Result<(), anyhow::Error> {
        // let mut count = 0;
        let mut reader = Reader::from_path("data/rating.csv")?;
        Ok(for result in reader.deserialize() {
            // count += 1;
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
            // if count % 1000000 == 0 {
            //     println!("{}", count);
            // }
            // println!("{}", id);

        })
    }

    pub(crate) fn read_jogadores(jogadores: &mut HashTable<u32, JogadorComRating>) -> Result<(), anyhow::Error> {
        let mut reader = Reader::from_path("data/players.csv")?;
        Ok(for result in reader.deserialize() {
            let record: Jogador = result?;
            jogadores.insert(&record.sofifa_id, JogadorComRating {
                sofifa_id: record.sofifa_id,
                name: record.name,
                player_positions: record.player_positions,
                rating: 0.0,
                rating_count: 0,
                tags: Vec::new(),
            })?;
        })
    }