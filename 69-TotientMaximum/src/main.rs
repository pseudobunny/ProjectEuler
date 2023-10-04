use custom_math_utilities::totient;

fn maximum_totient_ratio(n: u64) -> Option<u64> {
    (2..=n)
        .map(|i| { (i, i as f64/totient(i) as f64)})
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
}

fn main() {
    println!("{}", maximum_totient_ratio(1_000_000).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(maximum_totient_ratio(10), Some(6));
    }

    #[test]
    fn q_case() {
        assert_eq!(maximum_totient_ratio(1_000_000), Some(510_510));
    }
}
