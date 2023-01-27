use custom_math_utilities::{big_num_to_digits, ConvergentsList};
use num::BigUint;

fn fractions_with_numerator_greater_than_denominator(expansions: usize) -> usize {
    let mut conv_list = ConvergentsList::new(1, 2, |_| BigUint::from(2_u64));

    (0..expansions)
        .map(|i| conv_list.get(i))
        .filter(|(_, h, k)| big_num_to_digits(h.clone()).len() > big_num_to_digits(k.clone()).len())
        .count()
}

fn main() {
    println!(
        "{}",
        fractions_with_numerator_greater_than_denominator(1000)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(fractions_with_numerator_greater_than_denominator(1000), 153);
    }
}
