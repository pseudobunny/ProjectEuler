use custom_math_utilities::check_primality;

fn check_if_sum_of_prime_and_square(i: u64, primes: &[u64]) -> bool {
    for p in primes.iter().filter(|&&p| p < i) {
        let mut j = 1;
        loop {
            let sum = p + 2*j*j;
            
            if sum == i {
                return true;
            }

            if sum > i {
                break;
            }
            
            j += 1; 
        }
    }

    false
}

fn smallest_break_of_goldbach_conjecture() -> u64 {
    let mut primes = vec![2,3,5,7];
    
    let mut result = 8;
    for i in 8.. {
        if check_primality(i) {
            primes.push(i);
            continue;
        }

        if i % 2 != 0 && !check_if_sum_of_prime_and_square(i, &primes) {
            result = i;
            break;
        }
    }

    result
}

fn main() {
    println!("{}", smallest_break_of_goldbach_conjecture())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(smallest_break_of_goldbach_conjecture(), 5777);
    }
}