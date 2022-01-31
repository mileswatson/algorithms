use std::collections::HashMap;

fn max_price_memo(rod: u32, prices: &[u32], memo: &mut HashMap<u32, u32>) -> u32 {
    if let Some(&price) = memo.get(&rod) {
        return price;
    }
    let max = prices
        .iter()
        .enumerate()
        .filter_map(|(l, p)| {
            if 0 < l && (l as u32) < rod {
                Some((l as u32, p))
            } else {
                None
            }
        })
        .map(|(l, p)| p + max_price_memo(rod - l, prices, memo))
        .max()
        .unwrap_or(0);
    if let Some(&price) = prices.get(rod as usize) {
        if price > max {
            memo.insert(rod, price);
            return price;
        }
    }
    memo.insert(rod, max);
    max
}

pub fn max_price(rod: u32, prices: &[u32]) -> u32 {
    max_price_memo(rod, prices, &mut HashMap::new())
}
