use std::collections::HashSet;

fn d(n: u64) -> u64 {
    let max: u64 = (n as f32).sqrt().ceil() as u64;
    let mut sum: u64 = 1;
    let mut i: u64 = 2;
    while i < max {
        if n % i == 0 {
            sum += i + (n / i);
        }
        i += 1;
    }
    if i * i == n {
        sum += i;
    }

    sum
}

const MAX_POSSIBLE_NON_ABUNDANT_SUM: u64 = 28123;
const MIN_ABUNDANT_NUM: u64 = 12;

fn sum_of_non_abundant_sums() -> u64 {
    // abundant num generation
    println!("Generating abundant nums");
    let abundant_nums: Vec<u64> = (MIN_ABUNDANT_NUM..MAX_POSSIBLE_NON_ABUNDANT_SUM)
        .filter(|&n| n < d(n))
        .collect();

    // abundant sum generation
    println!("Generating abundant sums");
    let abundant_sums = abundant_nums.iter().enumerate().flat_map(|(i, n)| {
        abundant_nums
            .iter()
            .skip(i)
            .map(|m| n + m)
            .filter(|&s| !(s > MAX_POSSIBLE_NON_ABUNDANT_SUM))
            .collect::<Vec<u64>>()
    });

    let abundant_sums_set: HashSet<u64> = HashSet::from_iter(abundant_sums);

    // non-abundant sum
    println!("Checking for non abundant nums");
    (1..MAX_POSSIBLE_NON_ABUNDANT_SUM)
        .filter(|n| !abundant_sums_set.contains(n))
        .sum()
}

fn main() {
    println!("{}", sum_of_non_abundant_sums())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d() {
        assert_eq!(d(12), 16);
    }

    #[test]
    fn q_case() {
        assert_eq!(sum_of_non_abundant_sums(), 4179871)
    }
}

