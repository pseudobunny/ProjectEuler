fn check_if_multiple(x: u64, divisors: &[u64]) -> bool {
    let mut all_divisors = true;

    for i in divisors.iter() {
        if x % i != 0 {
            all_divisors = false;
            break;
        }
    }

    all_divisors
}

fn find_smallest_multiple(factors: &[u64]) -> u64 {
    let factors = filter_factors(factors);

    let largest_factor = *factors.last().unwrap();
    let mut smallest_multiple = largest_factor;

    while !check_if_multiple(smallest_multiple, &factors) {
        smallest_multiple += largest_factor;
    }

    smallest_multiple
}

fn filter_factors(factors: &[u64]) -> Vec<u64> {
    factors
        .iter()
        .filter(|&&f| factors.iter().filter(|&&f2| f2 % f == 0).count() == 1)
        .copied()
        .collect::<Vec<u64>>()
}

fn main() {
    let factors = (1..=20).collect::<Vec<u64>>();

    println!("{}", find_smallest_multiple(&factors))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(
            find_smallest_multiple(&(1..=10).collect::<Vec<u64>>()),
            2520
        )
    }

    #[test]
    fn q_case() {
        assert_eq!(
            find_smallest_multiple(&(1..=20).collect::<Vec<u64>>()),
            232792560
        )
    }
}
