use std::f64::consts::PI;

use itertools::Itertools;
use num::BigInt;
use prime_factorization::Factorization;

// TODO: Make a more generic version of this that is used for both this and generated_func_list
fn pentagonal_number(k: i64) -> i64 {
    (k * (3 * k - 1)) / 2
}

fn generate_k(index: u32) -> i64 {
    ((index + 1) / 2) as i64 * (-1_i64).pow(index - 1)
}

pub struct PartionSolver {
    pub partitions: Vec<BigInt>,
}

impl PartionSolver {
    pub fn new() -> PartionSolver {
        PartionSolver {
            partitions: vec![num::one()],
        }
    }

    pub fn partition(&mut self, target: usize) -> BigInt {
        let current_len = self.partitions.len();

        if target < current_len {
            return self.partitions[target].clone();
        }

        for n in current_len..=target {
            self.partitions.push(num::zero());

            for i in 1.. {
                let k = generate_k(i);
                let pentagonal = pentagonal_number(k);

                // ensures we stay within usize bounds
                if pentagonal as usize > n {
                    break;
                }

                let current_partition = &self.partitions[n - pentagonal as usize].clone();

                self.partitions[n as usize] +=
                    (-1_i64).pow((k + 1).abs() as u32) * current_partition;
            }
        }

        self.partitions[target].clone()
    }
}

pub fn partition_approximation(n: f64) -> f64 {
    let coefficient = 1_f64 / (4_f64 * n * 3_f64.sqrt());
    let exp_part = PI * ((2_f64 * n) / 3_f64).sqrt();

    coefficient * exp_part.exp()
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
        let mut solver = PartionSolver::new();

        let exact_partitions = vec![
            1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627,
            792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604,
        ];

        for (i, &val) in exact_partitions.iter().enumerate() {
            assert_eq!(solver.partition(i), BigInt::from(val));
        }
    }

    #[test]
    fn test_prime_partition() {
        assert_eq!(prime_partition(10), 5)
    }
}
