use itertools::Itertools;

fn nth_lexicographic_permutation(n: usize) -> u64 {
    let digits = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut permutations: Vec<u64> = digits
        .iter()
        .copied()
        .permutations(digits.len())
        .unique()
        .map(|p| p.join("").parse::<u64>().expect("Could not parse num"))
        .collect();

    permutations.sort();

    permutations[n - 1]
}

fn main() {
    println!("{}", nth_lexicographic_permutation(1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(nth_lexicographic_permutation(1_000_000), 2783915460)
    }
}

