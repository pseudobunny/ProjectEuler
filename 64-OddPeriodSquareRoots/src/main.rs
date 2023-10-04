use custom_math_utilities::SqrtContinuousFractionCoefficients;

fn num_odd_periods(n: u64) -> usize {
    (0..=n)
        .map(SqrtContinuousFractionCoefficients::new)
        .map(|coefficients| coefficients.period.len())
        .filter(|len| len % 2 != 0)
        .count()
}

fn main() {
    println!("{}", num_odd_periods(10_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(num_odd_periods(13), 4)
    }

    #[test]
    fn q_case() {
        assert_eq!(num_odd_periods(10_000), 1322)
    }
}