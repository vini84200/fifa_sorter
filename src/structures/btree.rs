use std::mem;
use std::mem::size_of;

#[derive(Debug)]
struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Node<K, V>>,
}

#[derive(Debug)]
pub struct BTree<K, V> {
    root: Node<K,V>,
    props: BTreeProps,

}

#[derive(Debug)]
struct BTreeProps;

const PAGE: usize = 2048;

impl<K, V> Node<K, V>
where
    K: Ord,
{
    const ORDER: usize = (PAGE - size_of::<Node<K, V>>()) / (size_of::<K>() + size_of::<V>());
    const MIN_KEYS: usize = (Self::ORDER - 1) / 2;
    const MAX_KEYS: usize = Self::ORDER - 1;

    const MID_KEYS: usize = (Self::ORDER - 1) / 2;

    fn new(_keys: Option<Vec<K>>, _values: Option<Vec<V>>, _children: Option<Vec<Node<K,V>>>) -> Self {
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
}

impl BTreeProps {
    fn new() -> Self {
        BTreeProps {}
    }

    fn is_filled<K: Ord + Copy, V>(&self, node: &Node<K, V>) -> bool {
        node.keys.len() == Node::<K, V>::MAX_KEYS
    }

    fn split_child<K: Ord + Copy + Default, V: Clone>(&self, parent: &mut Node<K, V>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[Node::<K, V>::MID_KEYS];
        let middle_value = child.values[Node::<K, V>::MID_KEYS].clone();

        let right_keys = match child.keys.split_off(Node::<K, V>::MID_KEYS ).split_first() {
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
            right_children
        );

        parent.keys.insert(child_index, middle_key);
        parent.values.insert(child_index, middle_value);
        parent.children.insert(child_index + 1, new_child);
    }

    fn insert_non_full<K: Ord + Copy + Default, V: Clone>(&self, node: &mut Node<K, V>, key: K, value: V) {
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

impl<K,V> BTree<K,V>
    where
        K: Ord + Copy + Default,
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
}


#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use super::*;

    #[derive(Clone)]
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
            write!(f, "BIG")
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
}
