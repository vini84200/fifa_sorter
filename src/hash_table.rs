use anyhow::{Result, anyhow};

#[derive(Clone, Debug)]
struct TableCell<K, V> {
  item: Option<(K, V)>,
}

impl<K, V> Default for TableCell<K, V> {
    fn default() -> Self {
        TableCell {
            item: None
        }
    }
}

#[derive(Clone, Debug)]
pub struct HashTable<K, V> {
  items: Vec<TableCell<K, V>>,
  count: usize,
  size: usize
}

pub trait Hashable {
  fn hash(&self) -> usize;
}

impl Hashable for String {
  fn hash(&self) -> usize {
    let p = 31;
    let mut acc = 0;
    for (i, c) in self.chars().enumerate() {
      acc += (p^i) * c as usize;
    }
    acc
  }
}

impl Hashable for u32 {
  fn hash(&self) -> usize {
    *self as usize
  }
}

impl<K, V> HashTable<K, V> 
  where 
    K: Clone + Default + Hashable + PartialEq,
    V: Clone + Default
{
  pub fn new(size: usize) -> Self {
    HashTable {
      items: vec![TableCell::default(); size],
      count: 0,
      size
    }
  }

  pub fn rehash(&self, hash: usize) -> usize {
    hash % self.size
  }

  pub fn insert(&mut self, key: &K, value: V) -> Result<()> {
    let hash = self.rehash(key.hash());
    if self.count == self.size {
      return Err(anyhow!("Hash table is full /{}", self.size));
    }

    for i in 0..self.size {
      if let Some(i) = self.items.get_mut((hash + i * 7 + i^2 * 13) % self.size) {
        if i.item.is_none() {
          i.item = Some((key.clone(), value));
          self.count += 1;
          return Ok(());
        }
      }
    }
    print!("{}", hash);
    Err(anyhow!("Hash table is full {}/{}", self.count, self.size))
  }

  pub fn get(&self, key: &K) -> Option<V> {
    let mut hash = self.rehash(key.hash());
    for c in 0..self.size {
      if let Some(i) = self.items.get((hash + c * 7 + c^2 * 13) % self.size) {
        if i.item.is_some() {
          if i.item.as_ref().unwrap().0 == *key {
            return Some(i.item.as_ref().unwrap().1.clone());
          }
        } else {
          return None;
        }
      }
    }
    None
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    let mut hash = self.rehash(key.hash());
    for c in 0..self.size {
      if let Some(i) = self.items.get((hash + c * 7 + c^2 * 13) % self.size) {
        if let Some(i) = &i.item {
          if i.0 == *key {
            hash = (hash + c * 7 + c^2 * 13) % self.size;
            break;
          }
        } else {
          return None;
        }
      }
    }
    if let Some(i) = self.items.get_mut(hash) {
      if let Some(i) = &mut i.item {
        return Some(&mut i.1);
      } else {
        return None;
      }
    } else {
      return None;
    }
  }

  pub fn get_mut_or_default(&mut self, key: &K) -> Result<&mut V> {
    if self.get(key).is_some(){
      Ok(self.get_mut(key).unwrap())
    } else {
      self.insert(key, Default::default())?;
      self.get_mut(key).ok_or(anyhow!("Could not get mut or default"))
    }
  }
 
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn insert() {
    let mut table = HashTable::new(101);
    assert! (table.insert(&"Peter Parker".to_string(), "SpiderMan".to_string()).is_ok());
    assert! (table.insert(&"Tony Stark".to_string(), "IronMan".to_string()).is_ok());
  }

  #[test]
  fn getting_elements() {
    let mut table = HashTable::new(3);
    assert! (table.insert(&"Peter Parker".to_string(), "SpiderMan".to_string()).is_ok());
    assert! (table.insert(&"Tony Stark".to_string(), "IronMan".to_string()).is_ok());

    println!("{:?}", table);

    assert_eq!(table.get(&"Peter Parker".to_string()), Some("SpiderMan".to_string()));
  }
}