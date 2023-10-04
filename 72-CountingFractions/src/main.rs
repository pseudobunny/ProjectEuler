use custom_math_utilities::totient;

fn num_unique_fractions(d_max: u64) -> u64 {
    (2..=d_max)
        .map(|d| totient(d))
        .sum()
}

fn main() {
    println!("{}", num_unique_fractions(1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(num_unique_fractions(8), 21);
    }

    #[test]
    fn q_case() {
        assert_eq!(num_unique_fractions(1_000_000), 303963552391);
    }
}
