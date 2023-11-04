use custom_math_utilities::{check_primality, digits_to_num, num_to_digits};
use itertools::Itertools;
use std::collections::HashSet;

fn index_permutations(len: usize) -> Vec<Vec<usize>> {
    (1..len)
        .map(|n| (0..len).permutations(n).unique().collect_vec())
        .collect_vec()
        .concat()
}

fn replace_digits_of_num_vec(digits: &[u64], indices: &[usize], replace_digit: u64) -> Vec<u64> {
    digits
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if indices.contains(&i) {
                replace_digit as u64
            } else {
                *v
            }
        })
        .collect_vec()
}

fn replace_digits_of_num_vec_with_one_through_nine(
    digits: &[u64],
    indices: &[usize],
    len: usize,
) -> Vec<u64> {
    (0..10)
        .map(|d| replace_digits_of_num_vec(digits, indices, d))
        .map(|v| digits_to_num(&v))
        .filter(|n| num_to_digits(*n).len() == len)
        .filter(|n| check_primality(*n))
        .collect_vec()
}

fn max_prime_value_family(num: u64) -> Vec<u64> {
    let num_digits = num_to_digits(num);
    let len = num_digits.len();
    let ind_perm = index_permutations(len);

    let num_digits_replaced = ind_perm
        .iter()
        .map(|ind| replace_digits_of_num_vec_with_one_through_nine(&num_digits, ind, len))
        .filter(|v| !v.is_empty())
        .collect_vec();

    // Deduping
    let mut replaced_set = HashSet::new();
    for n_d_r in num_digits_replaced {
        replaced_set.insert(n_d_r);
    }

    let mut replaced_vec = replaced_set.iter().collect_vec();
    replaced_vec.sort_by_key(|b| std::cmp::Reverse(b.len()));

    if replaced_vec.is_empty() {
        vec![]
    } else {
        replaced_vec[0].clone()
    }
}

fn find_primes_of_family(family: usize) -> Vec<u64> {
    let mut n = 9;

    loop {
        n += 1;
        // We can cut processing time by skipping any values that aren't prime
        // Reasoning - with how I set up the check, if it is in the prime family, it will find the
        // rest
        if !check_primality(n) {
            continue;
        }

        let prime_family = max_prime_value_family(n);

        if prime_family.len() == family {
            break prime_family;
        }
    }
}

fn main() {
    return println!("{:?}", find_primes_of_family(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(find_primes_of_family(6)[0], 13);
        assert_eq!(find_primes_of_family(7)[0], 56003);
    }

    #[test]
    fn q_case() {
        assert_eq!(find_primes_of_family(8)[0], 121313);
    }
}
