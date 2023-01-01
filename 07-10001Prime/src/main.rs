use custom_math_utilities::check_primality;

fn nth_prime(prime_index: u64) -> u64 {
    let mut counter = 0;
    let mut curr_n = 1;

    while counter < prime_index {
        curr_n += 1;
        if check_primality(curr_n) {
            counter += 1;
        }
    }

    curr_n
}

fn main() {
    println!("{}", nth_prime(10_001));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
    }

    #[test]
    fn q_case() {
        assert_eq!(nth_prime(10_001), 104743)
    }
}
