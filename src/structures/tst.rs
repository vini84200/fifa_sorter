// Ternary Search tree
use anyhow::Result;

#[derive(Default)]
struct TstNode<T> where T: Default {
    esq: Option<Box<TstNode<T>>>,
    dir: Option<Box<TstNode<T>>>,
    next: Option<Box<TstNode<T>>>,
    content: Option<T>,
    c: char,

}

impl<T> TstNode<T> where T: Default {
    fn insert(&mut self, word: &String, content: T) -> Result<()> {
        self._insert(word, content, 0)?;
        Ok(())
    }

    fn _insert(&mut self, word: &String, content: T, scanned: usize) -> Result<()> {
  
        if let Some(c) = word.chars().nth(scanned as usize) {
            if self.c == c {
                self._insert(word, content, scanned + 1)?;
            } else if c < self.c {
                if let Some(&mut a) =  &self.esq {
                    a._insert(word, content, scanned)?;
                } else {
                    self.esq = Some(Box::new(TstNode::<T> {
                        c,
                        .. Default::default()
                    }));
                    self.esq._insert(word, content, scanned + 1)?;
                }
            } else {

            }
        }

        Ok(())
    }

    fn get(&self, word: String) -> Option<T> {
        None
    }
    
}