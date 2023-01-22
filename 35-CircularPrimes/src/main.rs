use custom_math_utilities::{check_primality, digits_to_num, num_to_digits};

fn check_primality_all_rotations(n: u64) -> bool {
    let digits = num_to_digits(n);
    let l = digits.len();

    let rotation_prime_count = (0..l)
        .map(|i| {
            digits
                .iter()
                .cycle()
                .skip(i)
                .take(l)
                .copied()
                .collect::<Vec<u64>>()
        })
        .map(|v| digits_to_num(v))
        .filter(|&n| check_primality(n))
        .count();

    rotation_prime_count == l
}

fn count_of_circular_primes_below(num: u64) -> usize {
    (100..num)
        .filter(|&i| check_primality_all_rotations(i))
        .count()
        + 13
}

fn main() {
    println!("{}", count_of_circular_primes_below(1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(count_of_circular_primes_below(1_000_000), 55)
    }
}
