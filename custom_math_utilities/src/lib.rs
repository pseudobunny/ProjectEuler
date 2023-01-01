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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primeality() {
        assert!(check_primality(104743));
    }
}
