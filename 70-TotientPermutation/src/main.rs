use custom_math_utilities::{num_to_digits, totient};

fn are_permutations(a: u64, b: u64) -> bool {
    let mut a_digits = num_to_digits(a);
    a_digits.sort();

    let mut b_digits = num_to_digits(b);
    b_digits.sort();

    a_digits == b_digits
}

fn minimum_totient_permutation_ratio(n: u64) -> Option<u64> {
    (2..=n)
        .map(|i| (i, totient(i)))
        .filter(|(i, t)| are_permutations(*i, *t))
        .map(|(i, t)| (i, i as f64 / t as f64))
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
}

fn main() {
    println!("{}", minimum_totient_permutation_ratio(10_000_000).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(minimum_totient_permutation_ratio(10_000_000), Some(8319823));
    }
}
