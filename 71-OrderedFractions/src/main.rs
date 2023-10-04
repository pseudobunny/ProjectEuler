use num::Rational64;

fn fractions_with_denominator_between(
    d: i64,
    max: Rational64,
    min: Option<Rational64>,
) -> Vec<Rational64> {
    let max_numerator = ((*max.numer() as f64 * d as f64) / *max.denom() as f64).ceil() as i64;
    let min_numerator = match min {
        Some(min_val) => {
            ((*min_val.numer() as f64 * d as f64) / *min_val.denom() as f64).ceil() as i64
        }
        None => 1,
    };

    (min_numerator..max_numerator)
        .map(|n| Rational64::new(n, d))
        .collect()
}

fn fraction_to_left_of_three_sevenths(d_max: i64) -> Option<Rational64> {
    let three_sevenths = Rational64::new(3, 7);

    (1..=d_max).fold(None, |to_left, d| {
        fractions_with_denominator_between(d, three_sevenths, to_left)
            .iter()
            .fold(to_left, |to_left, &fraction| {
                to_left.map_or(Some(fraction), |left_fraction| {
                    if fraction > left_fraction {
                        Some(fraction)
                    } else {
                        to_left
                    }
                })
            })
    })
}

fn main() {
    println!("{}", fraction_to_left_of_three_sevenths(1_000_000).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(
            fraction_to_left_of_three_sevenths(8),
            Some(Rational64::new(2, 5))
        );
    }

    #[test]
    fn q_case() {
        assert_eq!(
            fraction_to_left_of_three_sevenths(1_000_000),
            Some(Rational64::new(428570, 999997))
        );
    }
}
