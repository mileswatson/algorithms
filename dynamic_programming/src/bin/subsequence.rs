use dynamic_programming::subsequence::longest_common_subsequence;

fn main() {
    let a: Vec<char> = "CCGTCAGTCGCG".chars().collect();
    let b: Vec<char> = "TGTTTCGGAATGCAA".chars().collect();
    let max = longest_common_subsequence(&a, &b);

    println!("{max}");
}
