use std::cmp::Ordering::Greater;

pub struct InsertionSorter {}

impl<T: Ord> crate::Sorter<T> for InsertionSorter {
    fn sort(&self, values: &mut [T]) {
        for i in 1..values.len() {
            for j in (0..i).rev() {
                match values[j].cmp(&values[j + 1]) {
                    Greater => values.swap(j, j + 1),
                    _ => break,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InsertionSorter;

    #[test]
    pub fn insertion_sort() {
        crate::tests::test(InsertionSorter {})
    }
}
