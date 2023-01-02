use num::{Num, NumCast};

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
}
