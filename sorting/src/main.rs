use sorting::{algorithms::HeapSorter, Sorter};

fn main() {
    let mut v = vec![1, 3, 2, 1, 5, 1];
    HeapSorter {}.sort(&mut v);
    println!("{v:?}");
}
