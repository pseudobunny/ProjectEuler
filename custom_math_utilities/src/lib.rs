use itertools::Itertools;
use num::{Integer, Float, Num, NumCast};
use prime_factorization::Factorization;

// PULL IN SUBMODULES

pub mod generated_func_list;
pub use crate::generated_func_list::hepta_number_list;
pub use crate::generated_func_list::hexa_number_list;
pub use crate::generated_func_list::octa_number_list;
pub use crate::generated_func_list::penta_number_list;
pub use crate::generated_func_list::square_number_list;
pub use crate::generated_func_list::triangle_number_list;
pub use crate::generated_func_list::GeneratedFuncList;

pub mod digit_operations;
pub use crate::digit_operations::big_num_to_digits;
pub use crate::digit_operations::digits_to_big_num;
pub use crate::digit_operations::digits_to_num;
pub use crate::digit_operations::digits_to_num_filter_ind;
pub use crate::digit_operations::num_to_digits;

pub mod convergents_list;
pub use crate::convergents_list::ConvergentsList;

pub mod sqrt_continuous_fraction;
pub use crate::sqrt_continuous_fraction::SqrtContinuousFractionCoefficients;

// GENERAL USEFUL FUNCTIONS (probably break into their own files eventually)

pub fn check_primality<N: Num + NumCast + PartialOrd + Copy>(n: N) -> bool {
    let nums: Vec<N> = (0..=6).map(|n| NumCast::from(n).unwrap()).collect();

    if n == nums[2] || n == nums[3] {
        return true;
    }
    if n <= nums[1] || n % nums[2] == nums[0] || n % nums[3] == nums[0] {
        return false;
    }
    let mut d = nums[5];
    while d * d <= n {
        if n % d == nums[0] || n % (d + nums[2]) == nums[0] {
            return false;
        }
        d = d + nums[6];
    }

    true
}

pub fn check_palindrome<T: ToString>(x: T) -> bool {
    let x_str = x.to_string();
    let n = x_str.len() / 2;

    x_str.bytes().take(n).eq(x_str.bytes().rev().take(n))
}

pub fn check_if_whole<F: Float>(n: F) -> bool {
    (n - n.round()).abs() < NumCast::from(0.00001).unwrap()
}

pub fn to_base(n: u32, base: u32) -> String {
    let mut result = vec![];
    let mut n = n; // get a copy of n but mutable

    loop {
        let m = n % base;
        n /= base;

        result.push(std::char::from_digit(m, base).unwrap());
        if n == 0 {
            break;
        }
    }

    result.into_iter().rev().collect()
}

pub fn relatively_prime_to(n: u64) -> Vec<u64> {
    (1..n).filter(|m| n.gcd(m) == 1).collect()
}

pub fn totient(n: u64) -> u64 {
    let prime_factors = Factorization::run(n).factors;
    println!("{:?}", prime_factors);
    let totient_f = prime_factors
        .iter()
        .unique()
        .map(|&factor| ((factor - 1) as f64 / factor as f64))
        .fold(n as f64, |acc, multipier| acc * multipier);

    totient_f.round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primeality() {
        assert!(check_primality(104743_u64));
        assert!(check_primality(104743_i64));
    }

    #[test]
    fn test_palindrome() {
        assert!(check_palindrome(10001));
    }

    #[test]
    fn test_if_whole() {
        assert!(check_if_whole(0.999999999999999));
        assert!(check_if_whole(1.000000000034453));
    }

    #[test]
    fn test_num_to_digits() {
        for i_num in num_to_digits(123456).iter().rev().enumerate() {
            assert_eq!(i_num.0 + 1, *i_num.1);
        }
    }

    #[test]
    fn test_digits_to_num() {
        assert_eq!(digits_to_num(&[1, 2, 3, 4, 5, 6]), 654321)
    }

    #[test]
    fn test_digits_to_num_filter_ind() {
        assert_eq!(digits_to_num_filter_ind(&[1, 2, 3, 4, 5, 6], 1), 65431)
    }

    #[test]
    fn test_to_base() {
        assert_eq!(to_base(7, 2), "111"); // have only needed base 2 so far
    }

    #[test]
    fn test_relatively_prime_to() {
        assert_eq!(relatively_prime_to(2), vec![1]);
        assert_eq!(relatively_prime_to(3), vec![1, 2]);
        assert_eq!(relatively_prime_to(4), vec![1, 3]);
        assert_eq!(relatively_prime_to(5), vec![1, 2, 3, 4]);
        assert_eq!(relatively_prime_to(6), vec![1, 5]);
        assert_eq!(relatively_prime_to(7), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(relatively_prime_to(8), vec![1, 3, 5, 7]);
        assert_eq!(relatively_prime_to(9), vec![1, 2, 4, 5, 7, 8]);
        assert_eq!(relatively_prime_to(10), vec![1, 3, 7, 9]);
    }

    #[test]
    fn test_totient() {
        assert_eq!(totient(2), 1);
        assert_eq!(totient(3), 2);
        assert_eq!(totient(4), 2);
        assert_eq!(totient(5), 4);
        assert_eq!(totient(6), 2);
        assert_eq!(totient(7), 6);
        assert_eq!(totient(8), 4);
        assert_eq!(totient(9), 6);
        assert_eq!(totient(10), 4);
    }
}
