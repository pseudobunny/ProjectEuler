use custom_math_utilities::{check_primality, digits_to_num};
use itertools::Itertools;

const DIGITS : [u64; 9] = [1,2,3,4,5,6,7,8,9];

fn generate_permutations(i: usize) -> Vec<Vec<u64>> {
    DIGITS[0..i]
            .iter()
            .permutations(i)
            .unique()
            .map(|v| v.into_iter().copied().collect::<Vec<u64>>())
            .collect()
}

fn pandigital_prime_one_to(n: usize) -> u64 {
    (1..n)
        .flat_map(|i| generate_permutations(i))
        .map(|perm| digits_to_num(&perm))
        .filter(|&n| check_primality(n))
        .max()
        .unwrap()
}

fn main() {
    println!("{}", pandigital_prime_one_to(9))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(pandigital_prime_one_to(9), 7652413);
    }
}
