pub mod binary;

pub trait SearchTree<T: Ord> {
    fn new() -> Self;
    fn insert(&mut self, value: T);
    fn delete(&mut self, value: &T) -> Option<T>;
    fn to_vec(self) -> Vec<T>;
}

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};

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
        for _ in 0..1000 {
            let index = thread_rng().gen_range(0..v.len());
            t.delete(&v.swap_remove(index));
        }
        v.sort_unstable();
        assert_eq!(v, t.to_vec())
    }
}
