use std::{
    cmp::Ordering::*,
    mem::{replace, take},
};

use super::SearchTree;
use BinaryTree::*;

struct BinaryTreeIter<'a, T>(Vec<&'a BinaryTree<T>>);

impl<'a, T> BinaryTreeIter<'a, T> {
    pub fn new(root: &'a BinaryTree<T>) -> BinaryTreeIter<T> {
        let mut iter = BinaryTreeIter(Vec::new());
        iter.set_next(root);
        iter
    }

    fn set_next(&mut self, mut tree: &'a BinaryTree<T>) {
        while let Node(_, left, _) = tree {
            self.0.push(tree);
            tree = left;
        }
    }
}

impl<'a, T> Iterator for BinaryTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop()? {
            Node(value, _, right) => {
                let v = value;
                self.set_next(right);
                Some(v)
            }
            Leaf => panic!(),
        }
    }
}

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

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        let x = BinaryTreeIter::<'a, _>::new(self);
        Box::new(x)
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
