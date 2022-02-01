use dynamic_programming::allocation::{max_allocation, TimeSpan};

fn main() {
    let bookings = vec![
        TimeSpan::new(1, 3),
        TimeSpan::new(2, 5),
        TimeSpan::new(4, 7),
        TimeSpan::new(1, 7),
        TimeSpan::new(5, 9),
        TimeSpan::new(8, 10),
        TimeSpan::new(9, 11),
        TimeSpan::new(11, 14),
        TimeSpan::new(13, 16),
    ];

    let max = max_allocation(&bookings, 1, 16);
    println!("Max activities: {max}");
}
