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

impl<T: Ord> BinaryTree<T> {
    fn new(value: T) -> BinaryTree<T> {
        BinaryTree::Value(
            value,
            Box::new(BinaryTree::Empty),
            Box::new(BinaryTree::Empty),
        )
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
