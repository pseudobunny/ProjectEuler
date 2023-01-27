use custom_math_utilities::{big_num_to_digits, ConvergentsList};
use num::BigUint;

fn sum_of_numerator_e_continued_fraction(n: usize) -> BigUint {
    let pattern = |i| {
        if (i + 2) % 3 == 1 {
            BigUint::from((i + 2) / 3 * 2)
        } else {
            BigUint::from(1_u64)
        }
    };

    let mut e_conv = ConvergentsList::new(2, 1, pattern);

    big_num_to_digits(e_conv.get(n - 1).1).iter().sum()
}

fn main() {
    println!("{}", sum_of_numerator_e_continued_fraction(100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(
            sum_of_numerator_e_continued_fraction(100),
            BigUint::from(272_u64)
        );
    }
}
