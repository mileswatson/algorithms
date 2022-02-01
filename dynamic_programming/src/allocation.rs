use std::collections::HashMap;

pub struct TimeSpan {
    start: u32,
    end: u32,
}

impl TimeSpan {
    pub fn new(start: u32, end: u32) -> TimeSpan {
        TimeSpan { start, end }
    }

    fn within(&self, start: u32, end: u32) -> bool {
        start <= self.start && self.end <= end
    }
}

fn max_allocation_dyn(
    bookings: &[TimeSpan],
    start: u32,
    end: u32,
    memo: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    if start == end {
        return 0;
    }
    if let Some(&num) = memo.get(&(start, end)) {
        return num;
    }
    bookings
        .iter()
        .filter(|b| b.within(start, end))
        .map(|b| {
            let left = max_allocation_dyn(bookings, start, b.start, memo);
            let right = max_allocation_dyn(bookings, b.end, end, memo);
            left + right + 1
        })
        .max()
        .unwrap_or(0)
}

pub fn max_allocation(bookings: &[TimeSpan], start: u32, end: u32) -> u32 {
    max_allocation_dyn(bookings, start, end, &mut HashMap::new())
}
