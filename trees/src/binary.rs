use std::{
    cmp::Ordering::*,
    mem::{replace, take},
};

use super::SearchTree;
use BinaryTree::*;

enum BinaryTree<T> {
    Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
    Leaf,
}

impl<T> Default for BinaryTree<T> {
    fn default() -> BinaryTree<T> {
        Leaf
    }
}

impl<T: Ord> BinaryTree<T> {
    fn new(value: T) -> BinaryTree<T> {
        Node(value, Box::new(Leaf), Box::new(Leaf))
    }

    fn value(self) -> Option<T> {
        match self {
            Node(val, ..) => Some(val),
            Leaf => None,
        }
    }

    fn delete_min(&mut self) -> Option<T> {
        match self {
            Leaf => None,
            Node(_, left, right) => match left.as_mut() {
                Leaf => {
                    let right = take(right);
                    replace(self, *right).value()
                }
                Node(..) => left.delete_min(),
            },
        }
    }

    fn fill_vec(self, v: &mut Vec<T>) {
        if let Node(val, left, right) = self {
            left.fill_vec(v);
            v.push(val);
            right.fill_vec(v);
        }
    }
}

impl<T: Ord> SearchTree<T> for BinaryTree<T> {
    fn new() -> BinaryTree<T> {
        Leaf
    }

    fn insert(&mut self, other: T) {
        match self {
            Leaf => {
                *self = BinaryTree::new(other);
            }
            Node(value, left, right) => {
                if &other <= value {
                    left.insert(other);
                } else {
                    right.insert(other);
                }
            }
        }
    }

    fn delete(&mut self, key: &T) -> Option<T> {
        match self {
            Leaf => None,
            Node(value, left, right) => match key.cmp(value) {
                Equal => match (left.as_mut(), right.as_mut()) {
                    (Leaf, Leaf) => take(self).value(),
                    (child, Leaf) | (Leaf, child) => {
                        let child = take(child);
                        replace(self, child).value()
                    }
                    (Node(..), Node(..)) => Some(replace(value, right.delete_min()?)),
                },
                Less => left.delete(key),
                Greater => right.delete(key),
            },
        }
    }

    fn to_vec(self) -> Vec<T> {
        let mut v = Vec::new();
        self.fill_vec(&mut v);
        v
    }
}

#[cfg(test)]
mod test {
    use crate::test::test;

    use super::BinaryTree;

    #[test]
    fn binary_tree_test() {
        test::<BinaryTree<i32>>();
    }
}
