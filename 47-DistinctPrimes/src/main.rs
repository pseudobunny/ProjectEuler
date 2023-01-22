use std::collections::HashSet;

fn prime_factorization_set(n: u64) -> HashSet<u64> {
    let mut prime_set = HashSet::new();
    let mut n = n;

    while n % 2  == 0 {
        prime_set.insert(2);
        n /= 2;
    }

    let sqrt_n = ((n as f64).sqrt()) as u64;
    for i in (3..=sqrt_n).step_by(2) {
        while n % i == 0 {
            prime_set.insert(i);
            n /= i;
        }
    }

    if n > 2 {
        prime_set.insert(n);
    }

    prime_set
}

fn consecutive_distinct_factors(distinct: usize) -> Vec<u64> {
    let mut consecutive = vec![];

    let mut i = 2;
    while consecutive.len() < distinct {
        if prime_factorization_set(i).len() == distinct {
            consecutive.push(i);
        } else {
            consecutive.clear()
        }

        i += 1;
    }

    consecutive
}

fn main() {
    println!("{:?}", consecutive_distinct_factors(4)[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(consecutive_distinct_factors(2)[0], 14);
        assert_eq!(consecutive_distinct_factors(3)[0], 644);
    }

    #[test]
    fn q_case() {
        assert_eq!(consecutive_distinct_factors(4)[0], 134043);
    }
}
