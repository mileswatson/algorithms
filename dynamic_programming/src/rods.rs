pub fn max_price(rod: u32, prices: &[u32]) -> u32 {
    let mut max_prices = vec![0];
    for i in 1..=rod {
        let max = prices
            .iter()
            .enumerate()
            .filter_map(|(l, p)| {
                if 0 < l && (l as u32) <= i {
                    Some(p + max_prices[i as usize - l])
                } else {
                    None
                }
            })
            .max()
            .unwrap();
        max_prices.push(max);
    }
    *max_prices.last().unwrap()
}
