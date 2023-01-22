use custom_math_utilities::check_primality;

fn find_next_prime(n: u64) -> u64 {
    let mut n = n + 1;
    while !check_primality(n) {
        n += 1
    }
    n
}

fn consecutive_prime_sum_that_is_prime_below(max_sum: u64) -> u64 {
    // Summation prime generation
    let mut primes = vec![2];
    while primes.iter().sum::<u64>() < max_sum {
        primes.push(find_next_prime(*primes.last().unwrap() as u64));
    }

    // Primes to check against
    let mut primes_check = primes.clone();
    primes_check.append(
        // While primes collects all primes that when consecutively summed, reach the maximum
        // this collects all primes that won't be used for summation, but could be a viable solution
        &mut (*primes.last().unwrap()..max_sum)
            .filter(|n| check_primality(*n))
            .collect::<Vec<u64>>(),
    );

    let prime_sums = primes.iter().fold(vec![], |mut acc, prime| {
        acc.push(*acc.last().unwrap_or(&0) + prime);
        acc
    });

    let pl = primes.len();
    (0..pl)
        .rev()
        .flat_map(|i| {
            (0..(pl - i))
                .map(|j| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .map(|(i, j)| prime_sums[j + i] - prime_sums[j] + primes[j])
        .filter(|sum| primes_check.contains(&sum))
        .next()
        .unwrap_or(0)
}

fn main() {
    println!("{}", consecutive_prime_sum_that_is_prime_below(1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(consecutive_prime_sum_that_is_prime_below(1_000), 953);
        assert_eq!(consecutive_prime_sum_that_is_prime_below(100), 41);
    }

    #[test]
    fn q_case() {
        assert_eq!(consecutive_prime_sum_that_is_prime_below(1_000_000), 997651);
    }
}
