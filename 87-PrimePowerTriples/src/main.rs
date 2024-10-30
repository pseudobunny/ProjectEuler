use custom_math_utilities::generate_primes_under;
use std::collections::HashSet;

fn num_of_prime_power_triples_under(n: u64) -> usize {
    let primes = generate_primes_under((n as f64).sqrt() as u64);

    let mut power_triples = HashSet::new();

    for x in &primes {
        for y in &primes {
            for z in &primes {
                let power_triple = x * x + y * y * y + z * z * z * z;

                if power_triple > n {
                    break;
                }

                power_triples.insert(power_triple);
            }
        }
    }

    power_triples.len()
}

fn main() {
    println!("{:?}", num_of_prime_power_triples_under(50_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(num_of_prime_power_triples_under(50), 4);
    }

    #[test]
    fn q_case() {
        assert_eq!(num_of_prime_power_triples_under(50_000_000), 1097343);
    }
}
