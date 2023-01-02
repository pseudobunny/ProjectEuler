use custom_math_utilities::check_primality;

fn sum_primes_under(n: u64) -> u64 {
    (2..n).filter(|i| check_primality(*i)).sum()
}

fn main() {
    println!("{}", sum_primes_under(2_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(sum_primes_under(10), 17)
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_primes_under(2_000_000), 142913828922)
    }
}
