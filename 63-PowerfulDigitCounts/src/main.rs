use custom_math_utilities::big_num_to_digits;
use num::BigUint;

fn powerful_digit_count_for_n(n: u64) -> usize {
    (1_usize..)
        .map(|i| (i, big_num_to_digits(BigUint::from(n).pow(i as u32)).len()))
        .take_while(|(i, n_digits)| i <= n_digits)
        .filter(|(i, n_digits)| i == n_digits)
        .count()
}

fn powerful_digit_count() -> usize {
    (1..10_u64).map(powerful_digit_count_for_n).sum()
}

fn main() {
    println!("{}", powerful_digit_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(powerful_digit_count(), 49)
    }
}
