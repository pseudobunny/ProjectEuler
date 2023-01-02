pub fn check_primality(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut d = 5;
    while d * d <= n {
        if n % d == 0 || n % (d + 2) == 0 {
            return false;
        }
        d += 6;
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
        assert!(check_primality(104743));
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
