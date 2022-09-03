use std::fmt::Debug;
use anyhow::{anyhow, Result};
use super::tst::Tst;


pub struct MultiTst<T> where T: Debug + Clone + Default{
    tst: Tst<Vec<T>>
}

impl<T> MultiTst<T>  where T: Debug + Clone + Default + PartialEq{
    pub fn new() -> Self {
        MultiTst {
            tst: Tst::new()
        }
    }

    pub fn insert(&mut self, key: String, value: T) -> Result<()> {
        let mut vec = self.tst.get(key.clone()).unwrap_or_else(|| Vec::new());
        if !vec.contains(&value) {
            vec.push(value);
        }
        self.tst.insert(&key, vec)?;
        Ok(())
    }

    pub fn get(&self, key: String) -> Option<Vec<T>> {
        self.tst.get(key)
    }

    pub fn find(&self, prefix: String) -> Vec<T> {
        self.tst.find_from_prefix(prefix).iter().map(|a| a.1.clone()).flatten().collect()
    }
}