use std::collections::HashMap;
#[derive(Clone)]
struct Matrix {
    dimensions: (u32, u32),
    cost: u32,
}

fn min_multiplications_dyn(start: u32, end: u32, memo: &mut HashMap<(u32, u32), Matrix>) -> Matrix {
    if let Some(min) = memo.get(&(start, end)) {
        return min.clone();
    }
    let min = (start..end)
        .map(|k| {
            let Matrix {
                dimensions: (left, mid),
                cost: left_cost,
            } = min_multiplications_dyn(start, k, memo);
            let Matrix {
                dimensions: (mid2, right),
                cost: right_cost,
            } = min_multiplications_dyn(k + 1, end, memo);
            assert_eq!(mid, mid2);
            Matrix {
                dimensions: (left, right),
                cost: left_cost + right_cost + left * mid * right,
            }
        })
        .min_by_key(|x| x.cost)
        .unwrap();
    memo.insert((start, end), min.clone());
    min
}

pub fn min_multiplications(dimensions: &[u32]) -> u32 {
    assert!(dimensions.len() >= 2);
    let mut memo = HashMap::<(u32, u32), Matrix>::new();
    for i in 0..dimensions.len() - 1 {
        memo.insert(
            (i as u32, i as u32),
            Matrix {
                dimensions: (dimensions[i], dimensions[i + 1]),
                cost: 0,
            },
        );
    }
    min_multiplications_dyn(0, dimensions.len() as u32 - 2, &mut memo).cost
}
