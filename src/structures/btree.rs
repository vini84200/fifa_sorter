use std::mem;
use std::mem::size_of;

#[derive(Debug, Clone)]
struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Node<K, V>>,
}

#[derive(Debug, Clone)]
pub struct BTree<K, V> {
    root: Node<K, V>,
    props: BTreeProps,

}

#[derive(Debug, Clone)]
struct BTreeProps;

const PAGE: usize = 2048;

impl<K, V> Node<K, V>
    where
        K: PartialOrd + Clone,
        V: Clone
{
    const ORDER: usize = (PAGE - size_of::<Node<K, V>>()) / (size_of::<K>() + size_of::<V>());
    const MIN_KEYS: usize = (Self::ORDER - 1) / 2;
    const MAX_KEYS: usize = Self::ORDER - 1;

    const MID_KEYS: usize = (Self::ORDER - 1) / 2;

    fn new(_keys: Option<Vec<K>>, _values: Option<Vec<V>>, _children: Option<Vec<Node<K, V>>>) -> Self {
        assert!(Self::ORDER > 2);
        Node {
            keys: _keys.unwrap_or(Vec::with_capacity(Self::MAX_KEYS)),
            values: _values.unwrap_or(Vec::with_capacity(Self::MAX_KEYS)),
            children: _children.unwrap_or(Vec::with_capacity(Self::MAX_KEYS + 1)),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get_all(&self) -> Vec<(K, V)> {
        let mut res = Vec::new();
        for i in 0..self.keys.len() {
            if self.is_leaf() {
                res.push((self.keys[i].clone(), self.values[i].clone()));
            } else {
                res.append(&mut self.children[i].get_all());
                res.push((self.keys[i].clone(), self.values[i].clone()));
            }
        }
        if !self.is_leaf() {
            res.append(&mut self.children[self.keys.len()].get_all());
        }
        res
    }

    pub fn get_n_greatest(&self, n: usize) -> Vec<(K, V)> {
        let mut res = Vec::new();
        for i in (0..self.keys.len()).rev() {
            if res.len() >= n {
                break;
            }
            if self.is_leaf() {
                res.push((self.keys[i].clone(), self.values[i].clone()));
            } else {
                res.append(&mut self.children[i + 1].get_n_greatest(n - res.len()));
                if res.len() < n {
                    res.push((self.keys[i].clone(), self.values[i].clone()));
                }
            }
        }
        if !self.is_leaf() && res.len() < n {
            res.append(&mut self.children[0].get_n_greatest(n - res.len()));
        }
        res
    }
}

impl BTreeProps {
    fn new() -> Self {
        BTreeProps {}
    }

    fn is_filled<K: PartialOrd + Copy, V: Clone>(&self, node: &Node<K, V>) -> bool {
        node.keys.len() == Node::<K, V>::MAX_KEYS
    }

    fn split_child<K: PartialOrd + Copy + Default, V: Clone>(&self, parent: &mut Node<K, V>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[Node::<K, V>::MID_KEYS];
        let middle_value = child.values[Node::<K, V>::MID_KEYS].clone();

        let right_keys = match child.keys.split_off(Node::<K, V>::MID_KEYS).split_first() {
            Some((_, keys)) => keys.to_vec(),
            None => Vec::with_capacity(Node::<K, V>::MAX_KEYS),
        };
        let right_values = match child.values.split_off(Node::<K, V>::MID_KEYS).split_first() {
            Some((_, values)) => values.to_vec(),
            None => Vec::with_capacity(Node::<K, V>::MAX_KEYS),
        };


        let right_children = if !child.is_leaf() {
            Some(child.children.split_off(Node::<K, V>::MID_KEYS + 1))
        } else {
            None
        };

        let new_child = Node::new(
            Some(right_keys),
            Some(right_values),
            right_children,
        );

        parent.keys.insert(child_index, middle_key);
        parent.values.insert(child_index, middle_value);
        parent.children.insert(child_index + 1, new_child);
    }

    fn insert_non_full<K: PartialOrd + Copy + Default, V: Clone>(&self, node: &mut Node<K, V>, key: K, value: V) {
        if node.is_leaf() {
            let mut i = node.keys.len();
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            node.keys.insert(i, key);
            node.values.insert(i, value);
        } else {
            let mut i = node.keys.len();
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            if self.is_filled(&node.children[i]) {
                self.split_child(node, i);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            self.insert_non_full(&mut node.children[i], key, value);
        }
    }
}

impl<K, V> Default for BTree<K, V>
    where
        K: PartialOrd + Copy + Default,
        V: Default + Clone
{
    fn default() -> Self {
        BTree {
            root: Node::new(None, None, None),
            props: BTreeProps::new(),
        }
    }
}

impl<K, V> BTree<K, V>
    where
        K: PartialOrd + Copy + Default,
        V: Default + Clone
{
    pub fn new() -> Self {
        BTree {
            root: Node::new(None, None, None),
            props: BTreeProps::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.props.is_filled(&self.root) {
            let mut new_root = Node::new(None, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.push(new_root);
            self.props.split_child(&mut self.root, 0);
        }
        self.props.insert_non_full(&mut self.root, key, value);
    }

    pub fn find(&self, key: K) -> Option<&V> {
        let mut node = &self.root;
        loop {
            let mut i = node.keys.len();
            while i > 0 && key <= node.keys[i - 1] {
                i -= 1;
            }
            if i < node.keys.len() && key == node.keys[i] {
                return Some(&node.values[i]);
            }
            if node.is_leaf() {
                return None;
            }
            node = &node.children[i];
        }
    }

    pub fn get_all(&self) -> Vec<(K, V)> {
        let mut result = Vec::new();
        let mut node = &self.root;
        loop {
            for i in 0..node.keys.len() {
                result.push((node.keys[i], node.values[i].clone()));
            }
            if node.is_leaf() {
                return result;
            }
            node = &node.children[node.keys.len()];
        }
    }

    pub fn get_greatest_n(&self, n: u32) -> Vec<V> {
        self.root.get_n_greatest(n as usize).into_iter().map(|(_, v)| v).collect()
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[derive(Clone, Eq, PartialEq)]
    struct BIG {
        a: [u8; 500],
    }

    impl Default for BIG {
        fn default() -> Self {
            BIG { a: [0; 500] }
        }
    }

    impl Debug for BIG {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BIG {{ a: [{}; 500] }}", self.a[0])
        }
    }

    #[test]
    fn insert() {
        let mut tree = BTree::new();
        tree.insert(1, 1);
        tree.insert(2, 2);
        tree.insert(3, 3);
        tree.insert(4, 4);

        println!("{:#?}", tree);
    }

    #[test]
    fn insert_big() {
        let mut tree = BTree::new();
        tree.insert(1, BIG { a: [0; 500] });
        tree.insert(2, BIG { a: [0; 500] });
        tree.insert(3, BIG { a: [0; 500] });
        tree.insert(4, BIG { a: [0; 500] });
        tree.insert(5, BIG { a: [0; 500] });
        tree.insert(7, BIG { a: [0; 500] });
        tree.insert(6, BIG { a: [0; 500] });
        tree.insert(8, BIG { a: [0; 500] });
        tree.insert(-1, BIG { a: [0; 500] });
        tree.insert(0, BIG { a: [0; 500] });
        println!("Order: {:#?} - MAX: {} - MIN : {}", Node::<i32, BIG>::ORDER, Node::<i32, BIG>::MAX_KEYS, Node::<i32, BIG>::MIN_KEYS);

        println!("{:#?}", tree);
    }

    #[test]
    fn find() {
        let mut tree = BTree::new();
        tree.insert(1, 1);
        tree.insert(2, 2);
        tree.insert(3, 3);
        tree.insert(4, 4);

        assert_eq!(tree.find(1), Some(&1));
        assert_eq!(tree.find(2), Some(&2));
        assert_eq!(tree.find(3), Some(&3));
        assert_eq!(tree.find(4), Some(&4));
        assert_eq!(tree.find(5), None);
    }

    #[test]
    fn find_big() {
        let mut tree = BTree::new();
        tree.insert(1, BIG { a: [2; 500] });
        tree.insert(2, BIG { a: [0; 500] });
        tree.insert(3, BIG { a: [0; 500] });
        tree.insert(4, BIG { a: [0; 500] });
        tree.insert(5, BIG { a: [0; 500] });
        tree.insert(7, BIG { a: [6; 500] });
        tree.insert(6, BIG { a: [0; 500] });
        tree.insert(8, BIG { a: [0; 500] });
        tree.insert(-1, BIG { a: [0; 500] });
        tree.insert(0, BIG { a: [0; 500] });
        println!("Order: {:#?} - MAX: {} - MIN : {}", Node::<i32, BIG>::ORDER, Node::<i32, BIG>::MAX_KEYS, Node::<i32, BIG>::MIN_KEYS);

        assert_eq!(tree.find(1), Some(&BIG { a: [2; 500] }));
        assert_eq!(tree.find(2), Some(&BIG { a: [0; 500] }));
        assert_eq!(tree.find(3), Some(&BIG { a: [0; 500] }));
        assert_eq!(tree.find(4), Some(&BIG { a: [0; 500] }));
        assert_eq!(tree.find(5), Some(&BIG { a: [0; 500] }));
        assert_eq!(tree.find(6), Some(&BIG { a: [0; 500] }));
        assert_eq!(tree.find(7), Some(&BIG { a: [6; 500] }));
        assert_eq!(tree.find(8), Some(&BIG { a: [0; 500] }));
        assert_eq!(tree.find(9), None);
    }

    #[test]
    fn get_greatest_n() {
        let mut tree = BTree::new();
        tree.insert(1, 1);
        tree.insert(2, 2);
        tree.insert(3, 3);
        tree.insert(4, 4);
        tree.insert(5, 5);

        assert_eq!(tree.get_greatest_n(3), vec![5, 4, 3]);
        assert_eq!(tree.get_greatest_n(5), vec![5, 4, 3, 2, 1]);
        assert_eq!(tree.get_greatest_n(6), vec![5, 4, 3, 2, 1]);
        assert_eq!(tree.get_greatest_n(0), vec![]);
    }

    #[test]
    fn get_greatest_n_on_empty_tree() {
        let tree = BTree::<i32, i32>::new();
        assert_eq!(tree.get_greatest_n(3), vec![]);
    }

    #[test]
    fn get_greatest_n_big() {
        let mut tree = BTree::new();
        tree.insert(1, BIG { a: [2; 500] });
        tree.insert(2, BIG { a: [0; 500] });
        tree.insert(3, BIG { a: [0; 500] });
        tree.insert(4, BIG { a: [0; 500] });
        tree.insert(5, BIG { a: [0; 500] });
        tree.insert(7, BIG { a: [6; 500] });
        tree.insert(6, BIG { a: [0; 500] });
        tree.insert(8, BIG { a: [9; 500] });
        tree.insert(-1, BIG { a: [0; 500] });
        tree.insert(0, BIG { a: [0; 500] });
        println!("Order: {:#?} - MAX: {} - MIN : {}", Node::<i32, BIG>::ORDER, Node::<i32, BIG>::MAX_KEYS, Node::<i32, BIG>::MIN_KEYS);

        assert_eq!(tree.get_greatest_n(3), vec![BIG { a: [9; 500] }, BIG { a: [6; 500] }, BIG { a: [0; 500] }]);
        assert_eq!(tree.get_greatest_n(5), vec![BIG { a: [9; 500] }, BIG { a: [6; 500] }, BIG { a: [0; 500] }, BIG { a: [0; 500] }, BIG { a: [0; 500] }]);
        assert_eq!(tree.get_greatest_n(0), vec![]);

        assert_eq!(tree.get_greatest_n(1), vec![BIG { a: [9; 500] }]);
    }
}
