use custom_math_utilities::{check_primality, digits_to_num, num_to_digits};

fn check_truncated(n: u64) -> bool {
    if !check_primality(n) {
        return false;
    }

    let digits = num_to_digits(n);
    let l = digits.len();

    let count_from_right = (1..l)
        .map(|i| digits.iter().skip(i).copied().collect::<Vec<u64>>())
        .map(|v| digits_to_num(&v))
        .filter(|n| check_primality(*n))
        .count();

    let count_from_left = (1..l)
        .rev()
        .map(|i| digits.iter().take(i).copied().collect::<Vec<u64>>())
        .map(|v| digits_to_num(&v))
        .filter(|n| check_primality(*n))
        .count();

    count_from_left == l - 1 && count_from_right == l - 1
}

fn sum_truncatable_primes() -> u64 {
    let mut trunc_primes = vec![];
    let mut i = 10;

    loop {
        if check_truncated(i) {
            trunc_primes.push(i)
        }

        if trunc_primes.len() > 10 {
            break;
        }

        i += 1;
    }

    trunc_primes.iter().sum()
}

fn main() {
    println!("{}", sum_truncatable_primes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_truncated() {
        assert!(check_truncated(3797));
    }

    #[test]
    fn test_sum_double_palindrome() {
        assert_eq!(sum_truncatable_primes(), 748317);
    }
}
