use std::collections::HashSet;

use custom_math_utilities::digit_factorial;

fn count_non_repeating_terms(n: u64) -> usize {
    let mut chain = HashSet::new();
    chain.insert(n);

    count_non_repeating_terms_inner(n, chain)
}

fn count_non_repeating_terms_inner(n: u64, mut chain: HashSet<u64>) -> usize {
    let next_in_chain = digit_factorial(n);

    if chain.contains(&next_in_chain) {
        chain.len()
    } else {
        chain.insert(next_in_chain);
        count_non_repeating_terms_inner(next_in_chain, chain)
    }
}

fn number_of_chains_with_x_terms(target_terms: usize, max_num: u64) -> usize {
    (0..max_num)
        .map(|n| count_non_repeating_terms(n))
        .filter(|&terms| target_terms == terms)
        .count()
}

fn main() {
    println!("{}", number_of_chains_with_x_terms(60, 1_000_000));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(
            count_non_repeating_terms(69),
            5
        );
    }

    #[test]
    fn q_case() {
        assert_eq!(
            number_of_chains_with_x_terms(60, 1_000_000),
            402
        );
    }
}