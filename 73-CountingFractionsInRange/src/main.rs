use itertools::Itertools;
use num::Rational64;

fn fractions_with_denominator_between(
    d: i64,
    max: Rational64,
    min: Rational64,
) -> Vec<Rational64> {
    let max_numerator = ((*max.numer() as f64 * d as f64) / *max.denom() as f64).ceil() as i64;
    let min_numerator = ((*min.numer() as f64 * d as f64) / *min.denom() as f64).floor() as i64 + 1;

    (min_numerator..max_numerator)
        .map(|n| Rational64::new(n, d).reduced())
        .collect()
}

fn fractions_between_third_and_half(d_max: i64) -> usize {
    let one_third = Rational64::new(1, 3);
    let one_half = Rational64::new(1, 2);

    (2..=d_max)
        .flat_map(|d| fractions_with_denominator_between(d, one_half, one_third))
        .unique()
        .count()
}

fn main() {
    println!("{}", fractions_between_third_and_half(12_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(
            fractions_between_third_and_half(8),
            3
        );
    }

    #[test]
    fn q_case() {
        assert_eq!(
            fractions_between_third_and_half(12_000),
            7295372
        );
    }
}