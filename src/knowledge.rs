use crate::{structures::{hash_table::HashTable, multi_tst::MultiTst}, models::{Jogador, JogadorComRating, User, Rating, Tag}};
use anyhow::{anyhow};

const JOGADOR_SIZE: usize = 22807;
const TAG_SIZE: usize = 438001;
const USER_SIZE: usize = 200001;
struct JogadoresDB {
    ht: HashTable<u32, JogadorComRating>,
    full_trie: MultiTst<u32>,
    tag: HashTable<String, u32>
}

impl JogadoresDB {
    fn new() -> Self {
        let ht = HashTable::new(JOGADOR_SIZE);
        let full_trie = MultiTst::new();
        let tag =  HashTable::new(TAG_SIZE);

        JogadoresDB {
            ht,
            full_trie,
            tag
        }
    }

    fn insert(&mut self, jogador: Jogador) -> Result<(), anyhow::Error> {
        self.ht.insert(&jogador.get_id(), JogadorComRating::from(jogador.clone()))?;
        self.full_trie.insert(jogador.get_name().clone(), jogador.get_id())?;

        Ok(())
    }

    fn get(&self, id: u32) -> Option<JogadorComRating> {
        self.ht.get(&id)
    }

    fn search(&self, name: String) -> Vec<JogadorComRating> {
        self.full_trie.find(name).iter().map(|a| self.get(*a).unwrap()).collect()
    }

    fn insert_tag(&mut self, tag: Tag) -> Result<(), anyhow::Error> {
        self.tag.insert(&tag.get_tag(), tag.get_id())?;
        self.ht.get_mut(&tag.get_id()).unwrap().add_tag(tag);
        Ok(())
    }

    fn add_rating(&mut self, rating: Rating) -> Result<(), anyhow::Error> {
        self.ht.get_mut(&rating.get_sofifa_id()).unwrap().add_rating(rating.get_rating());
        Ok(())
    }
}

struct UsersDB {
    ht: HashTable<u32, User>,
}

impl UsersDB {
    fn new() -> Self {
        let ht = HashTable::new(USER_SIZE);

        UsersDB {
            ht,
        }
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

impl DB {
    pub fn new() -> Self {
        let jogadores = JogadoresDB::new();
        let users = UsersDB::new();

        DB {
            jogadores,
            users,
        }
    }

    pub fn insert_jogador(&mut self, jogador: Jogador) -> Result<(), anyhow::Error> {
        self.jogadores.insert(jogador)?;

        Ok(())
    }

    pub fn insert_user(&mut self, user: User) -> Result<(), anyhow::Error> {
        self.users.insert(user)?;

        Ok(())
    }

    pub fn insert_rating(&mut self, rating: Rating) -> Result<(), anyhow::Error> {
        if let Some(user) = self.users.get_mut(rating.get_user_id()) {
            user.add_rating(&rating);
        } else {
            self.users.insert(
                User::from_rating(rating.clone())
            )?;
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
}