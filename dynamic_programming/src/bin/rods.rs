use dynamic_programming::rods::max_price;

fn main() {
    let prices = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
    let size = 8;
    let price = max_price(size, &prices);
    println!("Max price of {size}: {price}")
}
