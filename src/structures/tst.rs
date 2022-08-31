// Ternary Search tree
use anyhow::Result;

#[derive(Default, Debug)]
struct TstNode<T> where T: Default + Debug {
    esq: Option<Box<TstNode<T>>>,
    dir: Option<Box<TstNode<T>>>,
    next: Option<Box<TstNode<T>>>,
    content: Option<T>,
    c: char,

}

impl<T> TstNode<T> where T: Default + Debug{
    fn insert(&mut self, word: &String, content: T) -> Result<()> {
        self._insert(word, content, 0)?;
        Ok(())
    }

    fn _insert(&mut self, word: &String, content: T, scanned: usize) -> Result<()> {
  
        if let Some(c) = word.chars().nth(scanned as usize) {
            if self.c == c {
                if scanned == word.len() -1{
                    self.content = Some(content);
                } else {
                    self.next._insert(word, content, scanned + 1)?;
                }
            } else if c < self.c {
                if let Some(&mut a) =  &self.esq {
                    a._insert(word, content, scanned)?;
                } else {
                    
                    self.esq = Some(Box::new(TstNode::<T> {
                        c,
                        .. Default::default()
                    }));
                    self.esq.unwrap()._insert(word, content, scanned + 1)?;
                }
            } else {
                if let Some(&mut a) =  &self.dir {
                    a._insert(word, content, scanned)?;
                } else {
                    self.dir = Some(Box::new(TstNode::<T> {
                        c,
                        .. Default::default()
                    }));
                    self.dir.unwrap()._insert(word, content, scanned + 1)?;
                }
            }
        }

        Ok(())
    }

    fn get(&self, word: String) -> Option<T> {
        None
    }

    fn _get(&self, word: String, scanned: usize) -> Option<T> {
        if let Some(c) = word.chars().nth(scanned as usize) {
            if self.c == c {
                if scanned == word.len()-1{
                    return self.content;
                } else {
                    self.next._get(word, scanned + 1)?;
                }
            } else if c < self.c {
                if let Some(&mut a) =  &self.esq {
                    a._insert(word, content, scanned)?;
                } else {
                    self.esq = Some(Box::new(TstNode::<T> {
                        c,
                        .. Default::default()
                    }));
                    self.esq.unwrap()._insert(word, content, scanned + 1)?;
                }
            } else {
                if let Some(&mut a) =  &self.dir {
                    a._insert(word, content, scanned)?;
                } else {
                    self.dir = Some(Box::new(TstNode::<T> {
                        c,
                        .. Default::default()
                    }));
                    self.dir.unwrap()._insert(word, content, scanned + 1)?;
                }
            }
        }
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
        println!("{:?}", tst);
    }
}