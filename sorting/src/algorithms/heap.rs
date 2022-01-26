pub struct HeapSorter {}

fn fix_root<T: Ord>(values: &mut [T], parent: usize) {
    let left = 2 * parent + 1;
    let right = 2 * parent + 2;

    if left >= values.len() {
        return;
    }

    let max_child = if right < values.len() && values[right] > values[left] {
        right
    } else {
        left
    };

    if values[parent] < values[max_child] {
        values.swap(parent, max_child);
        fix_root(values, max_child);
    }
}

fn construct_max_heap<T: Ord>(values: &mut [T], n: usize) {
    let start = (1 << n) - 1;
    let end = (1 << (n + 1)) - 1;
    if end < values.len() {
        construct_max_heap(values, n + 1);
        for i in start..end {
            fix_root(values, i);
        }
    }
}

impl<T: Ord> crate::Sorter<T> for HeapSorter {
    fn sort(&self, values: &mut [T]) {
        construct_max_heap(values, 0);
        for i in (1..values.len()).rev() {
            values.swap(0, i);
            fix_root(&mut values[0..i], 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HeapSorter;

    #[test]
    pub fn heap_sort() {
        crate::tests::test(HeapSorter {})
    }
}
