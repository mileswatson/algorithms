use std::{cmp::max, collections::HashMap};

fn longest_common_subsequence_dyn<T: PartialEq>(
    a: &[T],
    b: &[T],
    memo: &mut HashMap<(usize, usize), u32>,
) -> u32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    if let Some(min) = memo.get(&(a.len(), b.len())) {
        return *min;
    }
    let max_length = if a[0] == b[0] {
        1 + longest_common_subsequence_dyn(&a[1..], &b[1..], memo)
    } else {
        let left = longest_common_subsequence_dyn(a, &b[1..], memo);
        let right = longest_common_subsequence_dyn(&a[1..], b, memo);
        max(left, right)
    };
    memo.insert((a.len(), b.len()), max_length);
    max_length
}

pub fn longest_common_subsequence<T: PartialEq>(a: &[T], b: &[T]) -> u32 {
    let mut memo = HashMap::<(usize, usize), u32>::new();
    longest_common_subsequence_dyn(a, b, &mut memo)
}
