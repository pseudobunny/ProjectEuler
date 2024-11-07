use custom_math_utilities::PartitionSolver;

fn least_n_for_which_p_divisible_by(d: u64) -> Option<usize> {
    let mut solver = PartitionSolver::new();

    (1..)
        .map(|n| 5 * n + 4) // Simplify by only calculating by Ramanujan congruences
        .find(|&n| solver.partition(n) % d == num::zero())
}

fn main() {
    println!("{}", least_n_for_which_p_divisible_by(1_000_000).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(least_n_for_which_p_divisible_by(1_000_000), Some(55374))
    }
}
