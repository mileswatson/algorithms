use std::mem::{replace, take};

use super::SearchTree;

struct BinaryTreeIter<'a, T>(Vec<&'a BinaryTree<T>>);

impl<'a, T> BinaryTreeIter<'a, T> {
    pub fn new(root: &'a BinaryTree<T>) -> BinaryTreeIter<T> {
        let mut iter = BinaryTreeIter(Vec::new());
        iter.set_next(root);
        iter
    }

    fn set_next(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::Value(_, left, _) = tree {
            self.0.push(tree);
            tree = left;
        }
    }
}

impl<'a, T> Iterator for BinaryTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop()? {
            BinaryTree::Value(value, _, right) => {
                let v = value;
                self.set_next(right);
                Some(v)
            }
            BinaryTree::Empty => panic!(),
        }
    }
}

enum BinaryTree<T> {
    Value(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
    Empty,
}

impl<T> Default for BinaryTree<T> {
    fn default() -> BinaryTree<T> {
        BinaryTree::Empty
    }
}

impl<T: Ord> BinaryTree<T> {
    fn new(value: T) -> BinaryTree<T> {
        BinaryTree::Value(
            value,
            Box::new(BinaryTree::Empty),
            Box::new(BinaryTree::Empty),
        )
    }

    fn value(self) -> Option<T> {
        match self {
            BinaryTree::Value(val, ..) => Some(val),
            BinaryTree::Empty => None,
        }
    }

    fn delete_min(&mut self) -> Option<T> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::Value(_, left, right) => match left.as_mut() {
                BinaryTree::Empty => {
                    let right = take(right);
                    replace(self, *right).value()
                }
                BinaryTree::Value(..) => left.delete_min(),
            },
        }
    }
}

impl<T: Ord> SearchTree<T> for BinaryTree<T> {
    fn new() -> BinaryTree<T> {
        BinaryTree::Empty
    }

    fn insert(&mut self, other: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::new(other);
            }
            BinaryTree::Value(value, left, right) => {
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
            BinaryTree::Empty => None,
            BinaryTree::Value(value, left, right) => {
                if key == value {
                    match (left.as_mut(), right.as_mut()) {
                        (BinaryTree::Empty, BinaryTree::Empty) => take(self).value(),
                        (child, BinaryTree::Empty) | (BinaryTree::Empty, child) => {
                            let child = take(child);
                            replace(self, child).value()
                        }
                        (BinaryTree::Value(..), BinaryTree::Value(..)) => {
                            Some(replace(value, right.delete_min()?))
                        }
                    }
                } else if key <= value {
                    left.delete(key)
                } else {
                    right.delete(key)
                }
            }
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
