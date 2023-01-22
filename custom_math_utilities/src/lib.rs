use core::iter::Sum;
use num::{traits::Pow, Num, NumCast};

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

pub fn check_palindrome(x: u64) -> bool {
    let x_str = x.to_string();
    let n = x_str.len() / 2;

    x_str.bytes().take(n).eq(x_str.bytes().rev().take(n))
}

pub fn check_if_whole(n: f32) -> bool {
    (n - n.trunc()).abs() < 0.00001_f32
}

pub fn num_to_digits<N: Num + NumCast + PartialOrd + Copy>(num: N) -> Vec<N> {
    let ten: N = NumCast::from(10).unwrap();

    fn ntd_inner<N: Num + NumCast + PartialOrd + Copy>(n: N, digits: &mut Vec<N>, ten_inner: N) {
        digits.push(n % ten_inner);
        if n >= ten_inner {
            ntd_inner(n / ten_inner, digits, ten_inner);
        }
    }

    let mut digits = vec![];
    ntd_inner(num, &mut digits, ten);

    digits
}

pub fn digits_to_num<N: Num + NumCast + PartialOrd + Copy + Sum>(digits: &[N]) -> N {
    digits
        .iter()
        .enumerate()
        .map(|(i, n)| *n * NumCast::from(10.pow(i as u32)).unwrap())
        .sum()
}

pub fn digits_to_num_filter_ind<N: Num + NumCast + PartialOrd + Copy + Sum>(
    digits: &[N],
    ind: usize,
) -> N {
    digits_to_num(
        &digits
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != ind)
            .map(|(_, n)| *n)
            .collect::<Vec<N>>(),
    )
}

pub fn to_base(n: u32, base: u32) -> String {
    let mut result = vec![];
    let mut n = n; // get a copy of n but mutable

    loop {
        let m = n % base;
        n = n / base;

        result.push(std::char::from_digit(m, base).unwrap());
        if n == 0 {
            break;
        }
    }

    result.into_iter().rev().collect()
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
        assert!(check_if_whole(0.999999999999));
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
        assert_eq!(digits_to_num(&vec![1, 2, 3, 4, 5, 6]), 654321)
    }

    #[test]
    fn test_digits_to_num_filter_ind() {
        assert_eq!(digits_to_num_filter_ind(&vec![1, 2, 3, 4, 5, 6], 1), 65431)
    }

    #[test]
    fn test_to_base() {
        assert_eq!(to_base(7, 2), "111"); // have only needed base 2 so far
    }
}
