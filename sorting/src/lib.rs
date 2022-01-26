mod algorithms;

use std::cmp::Ord;

pub trait Sorter<T: Ord> {
    fn sort(&self, values: &mut [T]);
}

#[cfg(test)]
pub mod tests {
    use super::Sorter;

    fn random_vec(size: usize) -> Vec<i32> {
        (0..size).map(|_| rand::random()).collect()
    }

    pub fn test<T: Sorter<i32>>(sorter: T) {
        for size in [2, 3, 10, 11, 15, 1000, 10000] {
            let v: &mut [i32] = &mut random_vec(size);
            sorter.sort(v);
            for i in 1..size {
                assert!(v[i - 1] <= v[i]);
            }
        }
    }
}
