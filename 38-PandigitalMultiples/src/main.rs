use custom_math_utilities::{digits_to_num, num_to_digits};
use std::collections::HashSet;

fn is_pandigital(n_a: &Vec<u64>) -> bool {
    HashSet::<u64>::from_iter(n_a.iter().copied()).len() == n_a.len()
}

fn digits_for_pandigital_construction(n: u64) -> Vec<u64> {
    let mut collect: Vec<u64> = vec![];

    for m in 1.. {
        let a = num_to_digits(n * m);
        collect = [a, collect].concat();

        if collect.len() > 8 {
            break;
        }
    }

    collect
}

fn max_pandigital() -> u64 {
    (192..100_000)
        .map(|n| digits_for_pandigital_construction(n))
        .filter(|v| !v.contains(&0) && is_pandigital(v))
        .map(|v| digits_to_num(&v))
        .max()
        .unwrap()
}

fn main() {
    println!("{}", max_pandigital())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(max_pandigital(), 932718654);
    }
}
