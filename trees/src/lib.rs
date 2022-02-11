pub mod binary;

pub trait SearchTree<T: Ord> {
    fn new() -> Self;
    fn insert(&mut self, value: T);
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>;
}

#[cfg(test)]
mod test {
    use super::SearchTree;

    fn random_vec(size: usize) -> Vec<i32> {
        (0..size).map(|_| rand::random()).collect()
    }

    pub fn test<T: SearchTree<i32>>() {
        let mut v = random_vec(10000);
        let mut t = T::new();
        for x in v.iter() {
            t.insert(*x);
        }
        v.sort_unstable();
        assert_eq!(v, t.iter().copied().collect::<Vec<_>>())
    }
}
