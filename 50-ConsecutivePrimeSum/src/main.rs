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

fn find_next_prime(n: u64) -> u64 {
    let mut n = n+1;
    while !check_primality(n) { n += 1 }
    n
}

fn main() {
    let max_sum = 1_000_000;
    
    // Summation prime generation
    let mut primes = vec![2];

    while primes.iter().sum::<u64>() < max_sum {
        primes.push(find_next_prime(*primes.last().unwrap() as u64));
    }
   
    // Primes to check against
    let mut primes_check = primes.clone();
    primes_check.append
        (
            &mut (*primes.last().unwrap()..max_sum)
                .filter(|n| check_primality(*n))
                .collect::<Vec<u64>>()
        );
    
    let mut prime_sums = Vec::<u64>::with_capacity(primes.len());
    for prime in primes.iter() {
        prime_sums.push(*prime_sums.last().unwrap_or(&0) + prime);
    }

    for i in (0..primes.len()).rev() {
        for j in 0..(primes.len()-i) {
            let sum = prime_sums[j+i] - prime_sums[j] + primes[j];
            if primes_check.contains(&sum) {
                println!("{}", sum);
                return;
            }
        }
    }
}
