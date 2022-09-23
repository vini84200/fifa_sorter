use std::fmt::Debug;

use anyhow::Result;

use super::tst::Tst;

#[derive(Clone, Debug, Default)]
pub struct MultiTst<T>
where
    T: Debug + Clone + Default,
{
    tst: Tst<Vec<T>>,
}

impl<T> MultiTst<T>
    where
        T: Debug + Clone + Default + PartialEq,
{
    pub fn new() -> Self {
        MultiTst { tst: Tst::new() }
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
        self.tst
            .find_from_prefix(prefix)
            .iter()
            .flat_map(|a| a.1.clone())
            .collect()
    }

    pub fn debug(&self) {
        self.tst.print_vertical();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut tst = MultiTst::new();
        tst.insert("key".to_string(), 1).unwrap();
        tst.insert("key".to_string(), 2).unwrap();
        tst.insert("key".to_string(), 1).unwrap();
        tst.insert("key".to_string(), 3).unwrap();
        tst.insert("key".to_string(), 1).unwrap();

        assert_eq!(tst.get("key".to_string()), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_find() {
        let mut tst = MultiTst::new();
        tst.insert("key".to_string(), 1).unwrap();
        tst.insert("key".to_string(), 2).unwrap();
        tst.insert("key".to_string(), 1).unwrap();
        tst.insert("key".to_string(), 3).unwrap();
        tst.insert("key".to_string(), 1).unwrap();

        assert_eq!(tst.find("ke".to_string()), vec![1, 2, 3]);
    }

    #[test]
    fn heavy_test() {
        let mut tst = MultiTst::new();
        for i in 0..1000 {
            tst.insert("key".to_string(), i).unwrap();
        }
        assert_eq!(tst.find("ke".to_string()).len(), 1000);
    }

    #[test]
    fn gen_test_find() {
        let mut tst = MultiTst::new();
        tst.insert("key".to_string(), 6).unwrap();
        for i in 0..1000 {
            let word = format!("key{}", i);
            tst.insert(word, i).unwrap();
        }
        tst.insert("pey".to_string(), 7).unwrap();
        tst.insert("ya".to_string(), 8).unwrap();
        tst.insert("y".to_string(), 9).unwrap();
        assert_eq!(tst.find("key77".to_string()).len(), 11);
        assert_eq!(tst.find("key".to_string()).len(), 1001);
        assert_eq!(tst.find("pey".to_string()).len(), 1);
        assert_eq!(tst.find("ya".to_string()).len(), 1);
        assert_eq!(tst.find("y".to_string()).len(), 2);
    }
}
