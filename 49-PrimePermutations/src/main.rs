use std::collections::HashSet;

fn check_primality(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n %3 == 0 {
        return false;
    }
    let mut d = 5;
    while d*d <= n {
        if n % d == 0 || n % (d + 2) == 0 {
            return false;
        }
        d += 6;
    }

    true
}

fn uint_to_digits(n: u64) -> HashSet<u64> {
    let mut digits = HashSet::new();
    let mut n = n;

    while n > 0 {
        digits.insert(n%10);
        n /= 10;
    }

    digits
}

fn check_if_perm(a: u64, b: u64, c: u64) -> bool {
    let a_d = uint_to_digits(a);
    let b_d = uint_to_digits(b);
    let c_d = uint_to_digits(c);

    a_d == b_d && b_d == c_d
}

fn main() {
    let mut results: Vec<Vec<u64>> = vec![];

    // Generate Primes
    let primes: Vec<u64> = (1000..=9999).filter(|n| check_primality(*n)).collect();
    let max_prime = *primes.last().unwrap();

    for p1 in primes.iter() {
        let p1 = *p1;

        for p2 in primes.iter().filter(|n| **n > p1 && ((2 * (**n)) - p1) <= max_prime) {
            let p2 = *p2;

            let diff = p2 - p1;
            
            match primes.iter().find(|n| **n == p2 + diff) {
                None => continue,
                Some(p3) => {
                    if check_if_perm(p1, p2, *p3) {
                        results.push(vec![p1, p2, *p3])
                    }
                }
            }
        } 
    }
    
    println!("{}{}{}", results[1][0], results[1][1], results[1][2])
}
