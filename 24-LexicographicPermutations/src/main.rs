use itertools::Itertools;

fn main() {
    let digits = vec!["0","1","2","3","4","5","6","7","8","9"];
    let mut permutations: Vec<u64> = digits.iter()
                                          .copied()
                                          .permutations(digits.len())
                                          .unique()
                                          .map(|p| p.join("").parse::<u64>().expect("Could not parse num"))
                                          .collect();

    permutations.sort();

    println!("{}", permutations[999_999])
}