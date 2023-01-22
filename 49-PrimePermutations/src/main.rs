use custom_math_utilities::{check_primality, num_to_digits};
use std::collections::HashSet;

fn check_if_perm(a: u64, b: u64, c: u64) -> bool {
    let a_d: HashSet<u64> = HashSet::from_iter(num_to_digits(a).into_iter());
    let b_d: HashSet<u64> = HashSet::from_iter(num_to_digits(b).into_iter());
    let c_d: HashSet<u64> = HashSet::from_iter(num_to_digits(c).into_iter());

    a_d == b_d && b_d == c_d
}

fn concatinated_prime_permutations() -> Vec<String> {
    let mut results: Vec<Vec<u64>> = vec![];

    // Generate Primes
    let primes: Vec<u64> = (1000..=9999).filter(|n| check_primality(*n)).collect();
    let max_prime = *primes.last().unwrap();

    for p1 in primes.iter() {
        let p1 = *p1;

        for p2 in primes
            .iter()
            .filter(|&&n| n > p1 && ((2 * n) - p1) <= max_prime)
        {
            let p2 = *p2;
            let diff = p2 - p1;

            match primes.iter().find(|&&n| n == p2 + diff) {
                None => continue,
                Some(p3) => {
                    if check_if_perm(p1, p2, *p3) {
                        results.push(vec![p1, p2, *p3])
                    }
                }
            }
        }
    }

    results
        .iter()
        .map(|v| format!("{}{}{}", v[0], v[1], v[2]))
        .collect()
}

fn main() {
    println!("{}", concatinated_prime_permutations()[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(
            concatinated_prime_permutations()[0],
            String::from("148748178147")
        );
    }

    #[test]
    fn q_case() {
        assert_eq!(
            concatinated_prime_permutations()[1],
            String::from("296962999629")
        );
    }
}
