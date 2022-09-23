use anyhow::anyhow;

use crate::{
    models::{Jogador, JogadorComRating, Rating, Tag, User},
    parser::Query,
    structures::{hash_table::HashTable, multi_tst::MultiTst},
};
use crate::structures::btree::BTree;

const JOGADOR_SIZE: usize = 22807;
const TAG_SIZE: usize = 438001;
const USER_SIZE: usize = 200001;

struct JogadoresDB {
    ht: HashTable<u32, JogadorComRating>,
    full_trie: MultiTst<u32>,
    tag: HashTable<String, Vec<u32>>,
    pos_ht: HashTable<String, BTree<f32, u32>>,
}

impl JogadoresDB {
    fn new() -> Self {
        let ht = HashTable::new(JOGADOR_SIZE);
        let full_trie = MultiTst::new();
        let tag = HashTable::new(TAG_SIZE);
        let pos_ht = HashTable::new(101);

        JogadoresDB {
            ht,
            full_trie,
            tag,
            pos_ht,
        }
    }

    fn insert(&mut self, jogador: Jogador) -> Result<(), anyhow::Error> {
        // println!("Inserting jogador {} - {}", jogador.get_id(), jogador.get_name());
        self.ht
            .insert(&jogador.get_id(), JogadorComRating::from(jogador.clone()))?;
        self.full_trie
            .insert(jogador.get_name().clone(), jogador.get_id())?;

        Ok(())
    }

    fn get(&self, id: u32) -> Option<JogadorComRating> {
        self.ht.get(&id)
    }

    fn search(&self, name: String) -> Vec<JogadorComRating> {
        self.full_trie
            .find(name)
            .iter()
            .map(|a| self.get(*a).unwrap())
            .collect()
    }

    fn insert_tag(&mut self, tag: Tag) -> Result<(), anyhow::Error> {
        if let Some(jogadores) = self.tag.get_mut(&tag.get_tag().to_lowercase()) {
            jogadores.push(tag.get_id());
        } else {
            self.tag
                .insert(&tag.get_tag().to_lowercase(), vec![tag.get_id()])?;
        }
        self.ht.get_mut(&tag.get_id()).unwrap().add_tag(tag);
        Ok(())
    }

    fn add_rating(&mut self, rating: Rating) -> Result<(), anyhow::Error> {
        self.ht
            .get_mut(&rating.get_sofifa_id())
            .unwrap()
            .add_rating(rating.get_rating());
        Ok(())
    }

    fn populate_pos_ht(&mut self) {
        self.ht.for_each(|_, jogador| {
            let positions = jogador.get_pos().player_positions.clone();
            let rating = jogador.get_rating();
            let id = jogador.get_sofifa_id();
            let count = jogador.get_rating_count();
            for pos in positions {
                if self.pos_ht.contains_key(&pos) {
                    if count > 1000 {
                        self.pos_ht.get_mut(&pos).unwrap().insert(rating, id);
                    }
                } else {
                    let mut btree = BTree::new();
                    if count > 1000 {
                        btree.insert(rating, id);
                    }
                    self.pos_ht.insert(&pos, btree).unwrap();
                }
            }
        });
    }
}

struct UsersDB {
    ht: HashTable<u32, User>,
}

impl UsersDB {
    fn new() -> Self {
        let ht = HashTable::new(USER_SIZE);

        UsersDB { ht }
    }

    fn insert(&mut self, user: User) -> Result<(), anyhow::Error> {
        self.ht.insert(&user.get_id(), user)?;

        Ok(())
    }

    fn get(&self, id: u32) -> Option<User> {
        self.ht.get(&id)
    }

    fn get_mut(&mut self, id: u32) -> Option<&mut User> {
        self.ht.get_mut(&id)
    }
}

pub struct DB {
    jogadores: JogadoresDB,
    users: UsersDB,
}

#[derive(Debug)]
pub enum QueryResult {
    Jogador(JogadorComRating),
    Jogadores(Vec<JogadorComRating>),
    User(User),
}

impl DB {
    pub fn new() -> Self {
        let jogadores = JogadoresDB::new();
        let users = UsersDB::new();

        DB { jogadores, users }
    }

    pub fn insert_jogador(&mut self, jogador: Jogador) -> Result<(), anyhow::Error> {
        self.jogadores.insert(jogador)?;

        Ok(())
    }

    pub fn insert_rating(&mut self, rating: Rating) -> Result<(), anyhow::Error> {
        if let Some(user) = self.users.get_mut(rating.get_user_id()) {
            user.add_rating(&rating);
        } else {
            self.users.insert(User::from_rating(rating.clone()))?;
        }
        self.jogadores.add_rating(rating)?;

        Ok(())
    }

    pub fn get_jogador(&self, id: u32) -> Option<JogadorComRating> {
        self.jogadores.get(id)
    }

    pub fn get_user(&self, id: u32) -> Option<User> {
        self.users.get(id)
    }

    pub fn search_jogador(&self, name: String) -> Vec<JogadorComRating> {
        self.jogadores.search(name)
    }

    pub fn insert_tag(&mut self, tag: Tag) -> Result<(), anyhow::Error> {
        self.jogadores.insert_tag(tag)?;

        Ok(())
    }

    pub fn finish_init(&mut self) {
        self.jogadores.populate_pos_ht();
    }

    pub fn run_query(&self, query: Query) -> Result<QueryResult, anyhow::Error> {
        match query {
            Query::Player(name) => {
                let jogadores = self.search_jogador(name);
                if jogadores.len() == 1 {
                    Ok(QueryResult::Jogador(jogadores[0].clone()))
                } else {
                    Ok(QueryResult::Jogadores(jogadores))
                }
            }
            Query::User(id) => {
                if let Some(user) = self.get_user(id) {
                    Ok(QueryResult::User(user))
                } else {
                    Err(anyhow!("User not found"))
                }
            }
            Query::Top(n, position) => {
                let jogadores = self
                    .jogadores
                    .pos_ht
                    .get(&position)
                    .unwrap()
                    .get_greatest_n(n as u32)
                    .iter()
                    .map(|a| self.jogadores.get(*a).unwrap())
                    .collect::<Vec<JogadorComRating>>();
                Ok(QueryResult::Jogadores(jogadores))
            }
            Query::Tags(tags) => {
                let mut tags_iter = tags.iter();
                let first_tag: &String = tags_iter.next().unwrap();
                let mut last_jogadores = self
                    .jogadores
                    .tag
                    .get(&first_tag.to_lowercase())
                    .iter()
                    .flatten()
                    .map(|a| self.jogadores.get(*a).unwrap())
                    .collect::<Vec<JogadorComRating>>();
                for tag in tags_iter {
                    let jogadores = self
                        .jogadores
                        .tag
                        .get(&tag.to_lowercase())
                        .iter()
                        .flatten()
                        .map(|a| self.jogadores.get(*a).unwrap())
                        .filter(|a| last_jogadores.contains(a))
                        .collect::<Vec<JogadorComRating>>();
                    last_jogadores = jogadores;
                }
                // Remover duplicados
                if last_jogadores.len() > 1 {
                    let mut jogadores = vec![last_jogadores[0].clone()];
                    for j in last_jogadores {
                        if !jogadores.contains(&j) {
                            jogadores.push(j);
                        }
                    }
                    Ok(QueryResult::Jogadores(jogadores))
                } else {
                    Ok(QueryResult::Jogadores(vec![]))
                }
            } // _ => Err(anyhow!("Query not implemented")),
        }
    }
}
