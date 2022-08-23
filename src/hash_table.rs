use anyhow::{Result, anyhow};

#[derive(Clone, Debug)]
struct TableCell<K, V> {
  item: Vec<(K, V)>
}

impl<K, V> Default for TableCell<K, V> {
    fn default() -> Self {
        TableCell {
            item: Vec::new()
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
    if let Some(i) = self.items.get_mut(hash) {
      i.item.push((key.clone(), value));
    }
    self.count += 1;
    Ok(())
  }

  pub fn get(&self, key: &K) -> Option<V> {
    let hash = self.rehash(key.hash());
    if let Some(i) = self.items.get(hash) {
      for a in &i.item {
        if a.0 == *key {
          return Some(a.1.clone());
        }
      }
    }
    None
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    let hash = self.rehash(key.hash());
    if let Some(i) = self.items.get_mut(hash) {
      i.item.iter_mut().find(|a| a.0 == *key).map(|a| &mut a.1)
    } else {
      None
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
    let mut table = HashTable::new(2);
    table.insert(&"Peter Parker".to_string(), "SpiderMan".to_string());
    table.insert(&"Tony Stark".to_string(), "IronMan".to_string());
  }

  #[test]
  fn getting_elements() {
    let mut table = HashTable::new(1);
    table.insert(&"Peter Parker".to_string(), "SpiderMan".to_string());
    table.insert(&"Tony Stark".to_string(), "IronMan".to_string());

    println!("{:?}", table);

    assert_eq!(table.get(&"Peter Parker".to_string()), Some("SpiderMan".to_string()));
  }
}