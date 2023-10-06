use itertools::Itertools;
use prime_factorization::Factorization;

// TODO: Make a more generic version of this that is used for both this and generated_func_list
fn pentagonal_number(k: i64) -> i64 {
    (k * (3 * k - 1)) / 2
}

fn generate_k(index: u32) -> i64 {
    ((index + 1) / 2) as i64 * (-1_i64).pow(index - 1)
}

pub fn partition(target: i64) -> i64 {
    let mut partitions = vec![1];

    for n in 1..=target {
        partitions.push(0);

        for i in 1.. {
            let k = generate_k(i);
            let pentagonal = pentagonal_number(k);

            // ensures we stay within usize bounds
            if pentagonal > n {
                break;
            }

            partitions[n as usize] +=
                (-1_i64).pow((k + 1).abs() as u32) * partitions[(n - pentagonal) as usize]
        }
    }

    partitions[target as usize]
}

fn sum_of_prime_factors(n: u64) -> u64 {
    Factorization::run(n).factors.iter().unique().sum()
}

pub fn prime_partition(target: u64) -> u64 {
    let mut partitions = vec![0, 0];

    for n in 2..=target {
        let n_sopf = sum_of_prime_factors(n);
        let recursive_addition: u64 = (1..n)
            .map(|j| sum_of_prime_factors(j) * partitions[(n - j) as usize])
            .sum();

        partitions.push((n_sopf + recursive_addition) / n);
    }

    partitions[target as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert_eq!(partition(5), 7)
    }

    #[test]
    fn test_prime_partition() {
        assert_eq!(prime_partition(10), 5)
    }
}