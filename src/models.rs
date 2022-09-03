use anyhow::anyhow;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Jogador {
    sofifa_id: u32,
    name: String,
    player_positions: String,
}

#[derive(Debug, Clone, Default)]
pub struct JogadorComRating {
    sofifa_id: u32,
    name: String,
    player_positions: String,
    rating: f32,
    rating_count: u32,
    tags: Vec<String>,
}

impl From<Jogador> for JogadorComRating {
    fn from(jogador: Jogador) -> Self {
        JogadorComRating {
            name: jogador.name,
            player_positions: jogador.player_positions,
            rating: 0.0,
            rating_count: 0,
            tags: Vec::new(),
            sofifa_id: jogador.sofifa_id,
        }
    }
}

impl JogadorComRating {
    pub fn new(name: String, player_positions: String, id: u32) -> Self {
        JogadorComRating {
            name,
            player_positions,
            rating: 0.0,
            rating_count: 0,
            tags: Vec::new(),
            sofifa_id: id,
        }
    }

    pub fn add_rating(&mut self, rating: f32) {
        self.rating = (self.rating * self.rating_count as f32 + rating)
            / (self.rating_count + 1) as f32;
        self.rating_count += 1;
    }

    pub fn add_tag(&mut self, tag: Tag) {
        if !self.tags.contains(&tag.get_tag()) {
            self.tags.push(tag.get_tag().clone());
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_sofifa_id(&self) -> u32 {
        self.sofifa_id
    }
}

impl Jogador {
    pub fn new(sofifa_id: u32, name: String, player_positions: String) -> Self {
        Jogador {
            sofifa_id,
            name,
            player_positions,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.sofifa_id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct Rating {
    user_id: u32,
    sofifa_id: u32,
    rating: f32,
}

impl Rating {
    fn new(user_id: u32, sofifa_id: u32, rating: f32) -> Self {
        Rating {
            user_id,
            sofifa_id,
            rating,
        }
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    pub fn get_sofifa_id(&self) -> u32 {
        self.sofifa_id
    }

    pub fn get_rating(&self) -> f32 {
        self.rating
    }
}

#[derive(Debug, Clone, Default)]
pub struct User {
    id: u32,
    ratings: Vec<Rating>,
}

impl User {
    pub fn new(ratings: Vec<Rating>, id: u32) -> Self {
        User { ratings, id }
    }

    pub fn from_rating(rating: Rating) -> Self {
        User {
            ratings: vec![rating.clone()],
            id: rating.get_user_id(),
        }
    }

    pub fn get_ratings(&self) -> &Vec<Rating> {
        &self.ratings
    }

    pub fn add_rating(&mut self, rating: &Rating) {
        self.ratings.push(rating.clone());
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Tag {
    sofifa_id: u32,
    tag: String,
}

impl Tag {
    pub fn new(sofifa_id: u32, tag: String) -> Self {
        Tag { sofifa_id, tag }
    }

    pub fn get_id(&self) -> u32 {
        self.sofifa_id
    }

    pub fn get_tag(&self) -> &String {
        &self.tag
    }
}
