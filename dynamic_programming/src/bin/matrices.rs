use dynamic_programming::matrices;

fn main() {
    let min = matrices::min_multiplications(&[5, 10, 20, 25, 5]);
    println!("Min multiplications: {min}")
}
