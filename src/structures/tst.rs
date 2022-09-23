use core::fmt::Debug;

// Ternary Search tree
use anyhow::Result;

#[derive(Debug, Clone)]
struct TstNode<T> where T: Default + Debug {
    esq: Option<Box<TstNode<T>>>,
    dir: Option<Box<TstNode<T>>>,
    next: Option<Box<TstNode<T>>>,
    content: Option<T>,
    c: char,

}

#[derive(Debug, Default, Clone)]
pub struct Tst<T> where T: Default + Debug + Clone {
    root: TstNode<T>,
}

impl<T> TstNode<T> where T: Default + Debug + Clone {
    pub fn insert(&mut self, word: &str, content: T) -> Result<()> {
        let word = word.to_lowercase();
        self._insert(&word, content, 0)?;
        Ok(())
    }

    fn _insert(&mut self, word: &String, content: T, scanned: usize) -> Result<()> {
        let last = scanned == word.len() - 1;

        if let Some(c) = word.chars().nth(scanned as usize) {
            if self.c == c {
                if last {
                    self.content = Some(content);
                } else if let Some(next) = &mut self.next {
                    next._insert(word, content, scanned + 1)?;
                } else {
                    let mut next = TstNode::default();
                    if let Some(c) = word.chars().nth(scanned as usize + 1) {
                        next.c = c;
                    }
                    self.next = Some(Box::new(next));
                    self.next.as_mut().unwrap()._insert(word, content, scanned + 1)?;
                }
            } else if c < self.c {
                if let Some(a) = &mut self.esq {
                    a._insert(word, content, scanned)?;
                } else {
                    self.esq = Some(Box::new(TstNode::<T> {
                        c,
                        content: if last { Some(content.clone()) } else { None },
                        ..Default::default()
                    }));
                    if !last {
                        self.esq.as_mut().unwrap()._insert(word, content, scanned)?;
                    }
                }
            } else if let Some(a) = &mut self.dir {
                a._insert(word, content, scanned)?;
            } else {
                self.dir = Some(Box::new(TstNode::<T> {
                    c,
                    content: if last { Some(content.clone()) } else { None },
                    ..Default::default()
                }));
                if !last {
                    self.dir.as_mut().unwrap()._insert(word, content, scanned)?;
                }
            }
        }

        Ok(())
    }

    pub fn get(&self, word: String) -> Option<T> {
        let word = Self::pre_process(word);
        self._get(word, 0)
    }

    fn _get(&self, word: String, scanned: usize) -> Option<T> {
        let last = scanned == word.len() - 1;
        if let Some(c) = word.chars().nth(scanned as usize) {
            if self.c == c {
                if last {
                    self.content.clone()
                } else {
                    if let Some(next) = &self.next {
                        next._get(word, scanned + 1)
                    } else { None }
                }
            } else if c < self.c {
                if let Some(a) = &self.esq {
                    a._get(word, scanned)
                } else {
                    None
                }
            } else {
                if let Some(a) = &self.dir {
                    a._get(word, scanned)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn find_from_prefix(&self, prefix: String) -> Vec<(String, T)> {
        let prefix = Self::pre_process(prefix);
        self._find_from_prefix(prefix, 0)
    }

    pub fn get_child_words(&self) -> Vec<(String, T)> {
        self._get_child_words("".to_string())
    }

    fn _get_child_words(&self, prefix: String) -> Vec<(String, T)> {
        let mut words = vec![];
        if let Some(content) = &self.content {
            words.push((prefix.clone() + &self.c.to_string(), content.clone()));
        }
        if let Some(next) = &self.next {
            words.append(&mut next._get_words(prefix.clone()));
        }
        words
    }

    pub fn get_words(&self) -> Vec<(String, T)> {
        self._get_words("".to_string())
    }

    fn _get_words(&self, prefix: String) -> Vec<(String, T)> {
        let mut words = vec![];
        if let Some(content) = &self.content {
            words.push((prefix.clone() + &self.c.to_string(), content.clone()));
        }
        if let Some(next) = &self.next {
            words.append(&mut next._get_words(prefix.clone() + &self.c.to_string()));
        }

        if let Some(esq) = &self.esq {
            words.append(&mut esq._get_words(prefix.clone()));
        }

        if let Some(dir) = &self.dir {
            words.append(&mut dir._get_words(prefix.clone()));
        }

        words
    }

    fn _find_from_prefix(&self, prefix: String, scanned: usize) -> Vec<(String, T)> {
        let last = scanned == prefix.len() - 1;
        if let Some(c) = prefix.chars().nth(scanned as usize) {
            if self.c == c {
                if last {
                    self._get_child_words(prefix)
                } else {
                    if let Some(next) = &self.next {
                        next._find_from_prefix(prefix, scanned + 1)
                    } else { vec![] }
                }
            } else if c < self.c {
                if let Some(a) = &self.esq {
                    a._find_from_prefix(prefix, scanned)
                } else {
                    vec![]
                }
            } else {
                if let Some(a) = &self.dir {
                    a._find_from_prefix(prefix, scanned)
                } else {
                    vec![]
                }
            }
        } else {
            vec![]
        }
    }

    fn _print_vertical(&self, level: usize) {
        if let Some(next) = &self.next {
            next._print_vertical(level + 1);
        }
        if let Some(dir) = &self.dir {
            dir._print_vertical(level + 1);
        }
        for _ in 0..level {
            print!(".");
        }
        println!("{} - {:?}", self.c, self.content);
        if let Some(esq) = &self.esq {
            esq._print_vertical(level + 1);
        }
    }

    fn pre_process(word: String) -> String {
        let word = word
            .to_lowercase()
            .trim()
            .to_string();
        word
    }
}

impl<T> Default for TstNode<T> where T: Clone + Debug + Default {
    fn default() -> Self {
        Self {
            c: ' ',
            content: None,
            next: None,
            esq: None,
            dir: None,
        }
    }
}

impl<T> Tst<T> where T: Clone + Default + Debug {
    pub fn new() -> Self {
        Self {
            root: TstNode::<T>::default()
        }
    }

    pub fn insert(&mut self, word: &String, content: T) -> Result<()> {
        self.root.insert(word, content)
    }

    pub fn get(&self, word: String) -> Option<T> {
        self.root.get(word)
    }

    pub fn find_from_prefix(&self, prefix: String) -> Vec<(String, T)> {
        self.root.find_from_prefix(prefix)
    }

    pub fn get_words(&self) -> Vec<(String, T)> {
        self.root.get_words()
    }

    pub fn print_vertical(&self) {
        self.root._print_vertical(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_tst() {
        let mut tst = TstNode::<i32>::default();
        tst.insert(&String::from("bola"), 1).unwrap();
        tst.insert(&String::from("hora"), 2).unwrap();
        tst.insert(&String::from("bala"), 2).unwrap();
        tst.insert(&String::from("terra"), 2).unwrap();
        tst.insert(&String::from("ter"), 2).unwrap();
        tst.insert(&String::from("terroso"), 2).unwrap();
        tst.insert(&String::from("Voar"), 2).unwrap();
        // tst._print_vertical(0);
    }

    #[test]
    fn getting_elements() {
        let mut tst = TstNode::<i32>::default();
        tst.insert(&String::from("bola"), 1).unwrap();
        tst.insert(&String::from("hora"), 2).unwrap();
        tst.insert(&String::from("bala"), 4).unwrap();

        assert_eq!(tst.get(String::from("bola")), Some(1));
        assert_eq!(tst.get(String::from("hora")), Some(2));
        assert_eq!(tst.get(String::from("bala")), Some(4));
        assert_eq!(tst.get(String::from("bolo")), None);
        assert_eq!(tst.get(String::from("b")), None);

        tst.insert(&String::from("bolo"), 5).unwrap();
        tst.insert(&String::from("bola"), 2).unwrap();

        assert_eq!(tst.get(String::from("bola")), Some(2));
        assert_eq!(tst.get(String::from("bolo")), Some(5));
    }

    #[test]
    fn get_from_prefix() {
        let mut tst = TstNode::<i32>::default();
        tst.insert(&String::from("bola"), 1).unwrap();
        tst.insert(&String::from("bolo"), 5).unwrap();
        tst.insert(&String::from("hora"), 2).unwrap();
        tst.insert(&String::from("bala"), 4).unwrap();

        assert_eq!(tst.find_from_prefix(String::from("bo")), vec![("bola".to_string(), 1), ("bolo".to_string(), 5)]);
        assert_eq!(tst.find_from_prefix(String::from("b")), vec![("bola".to_string(), 1), ("bolo".to_string(), 5), ("bala".to_string(), 4)]);
        assert_eq!(tst.find_from_prefix(String::from("h")), vec![("hora".to_string(), 2)]);
    }

    #[test]
    fn get_words() {
        let mut tst = TstNode::<i32>::default();
        tst.insert(&String::from("bola"), 1).unwrap();
        tst.insert(&String::from("bolo"), 5).unwrap();
        tst.insert(&String::from("hora"), 2).unwrap();
        tst.insert(&String::from("bala"), 4).unwrap();

        // assert_eq!(tst.get_words("".to_string()), vec![("bola".to_string(), 1), ("bolo".to_string(), 5), ("hora".to_string(), 2), ("bala".to_string(), 4)]);
        // Asserts with contains
        assert!(tst.get_words().contains(&("bola".to_string(), 1)));
        assert!(tst.get_words().contains(&("bolo".to_string(), 5)));
        assert!(tst.get_words().contains(&("hora".to_string(), 2)));
        assert!(tst.get_words().contains(&("bala".to_string(), 4)));
    }

    #[test]
    fn tst_struct() {
        let mut tst = Tst::<i32>::new();
        tst.insert(&String::from("bola"), 1).unwrap();
        tst.insert(&String::from("bolo"), 5).unwrap();
        tst.insert(&String::from("hora"), 2).unwrap();
        tst.insert(&String::from("bala"), 4).unwrap();

        assert_eq!(tst.get(String::from("bola")), Some(1));
        assert_eq!(tst.get(String::from("hora")), Some(2));
        assert_eq!(tst.get(String::from("bala")), Some(4));
        assert_eq!(tst.get(String::from("bolo")), Some(5));
        assert_eq!(tst.get(String::from("bolo")), Some(5));
        assert_eq!(tst.get(String::from("b")), None);

        tst.insert(&String::from("bolo"), 5).unwrap();
        tst.insert(&String::from("bola"), 2).unwrap();

        assert_eq!(tst.get(String::from("bola")), Some(2));
        assert_eq!(tst.get(String::from("bolo")), Some(5));

        assert_eq!(tst.find_from_prefix(String::from("bo")), vec![("bola".to_string(), 2), ("bolo".to_string(), 5)]);
        assert_eq!(tst.find_from_prefix(String::from("b")), vec![("bola".to_string(), 2), ("bolo".to_string(), 5), ("bala".to_string(), 4)]);
        assert_eq!(tst.find_from_prefix(String::from("h")), vec![("hora".to_string(), 2)]);

        assert!(tst.get_words().contains(&("bola".to_string(), 2)));
        assert!(tst.get_words().contains(&("bolo".to_string(), 5)));
        assert!(tst.get_words().contains(&("hora".to_string(), 2)));
        assert!(tst.get_words().contains(&("bala".to_string(), 4)));
    }

    #[test]
    fn test_case_nonsesitive() {
        let mut tst = Tst::<i32>::new();
        tst.insert(&String::from("bola"), 1).unwrap();
        tst.insert(&String::from("bOlo"), 5).unwrap();
        tst.insert(&String::from("hora"), 2).unwrap();

        assert_eq!(tst.get(String::from("Bola")), Some(1));
        assert_eq!(tst.get(String::from("HoRa")), Some(2));
        assert_eq!(tst.get(String::from("bala")), None);
        assert_eq!(tst.get(String::from("BOLO")), Some(5));

        assert_eq!(tst.find_from_prefix(String::from("BO")), vec![("bola".to_string(), 1), ("bolo".to_string(), 5)]);
    }
}