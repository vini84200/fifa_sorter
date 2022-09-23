use std::fmt::{Display, Formatter};

use serde::Deserialize;
use tabled::Tabled;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Jogador {
    sofifa_id: u32,
    name: String,
    player_positions: String,
}

#[derive(Debug, Clone, Default)]
struct Tags(Vec<String>);

#[derive(Debug, Clone, Default)]
pub struct Positons {
    pub player_positions: Vec<String>,
}

#[derive(Debug, Clone, Default, Tabled)]
pub struct JogadorComRating {
    id: u32,
    nome: String,
    posicoes: Positons,
    nota: f32,
    avaliacoes: u32,
    tags: Tags,
}

impl Display for Positons {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.player_positions.join(", "))
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tags = String::new();
        for tag in self.0.iter() {
            tags.push_str(&format!("'{}', ", tag));
        }
        write!(f, "{}", tags)
    }
}

impl PartialEq for JogadorComRating {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<Jogador> for JogadorComRating {
    fn from(jogador: Jogador) -> Self {
        JogadorComRating {
            nome: jogador.name,
            posicoes: Positons {
                player_positions: jogador
                    .player_positions
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            nota: 0.0,
            avaliacoes: 0,
            tags: Tags::default(),
            id: jogador.sofifa_id,
        }
    }
}

impl JogadorComRating {
    pub fn add_rating(&mut self, rating: f32) {
        self.nota = (self.nota * self.avaliacoes as f32 + rating) / (self.avaliacoes + 1) as f32;
        self.avaliacoes += 1;
    }

    pub fn add_tag(&mut self, tag: Tag) {
        if !self.tags.0.contains(tag.get_tag()) {
            self.tags.0.push(tag.get_tag().clone());
        }
    }

    pub fn get_name(&self) -> &String {
        &self.nome
    }

    pub fn get_sofifa_id(&self) -> u32 {
        self.id
    }

    pub fn get_rating(&self) -> f32 {
        self.nota
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags.0
    }

    pub fn get_pos(&self) -> &Positons {
        &self.posicoes
    }

    pub fn get_rating_count(&self) -> u32 {
        self.avaliacoes
    }
}

impl Jogador {
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
    pub fn get_id(&self) -> u32 {
        self.sofifa_id
    }

    pub fn get_tag(&self) -> &String {
        &self.tag
    }
}
