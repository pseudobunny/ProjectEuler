use itertools::Itertools;
use std::collections::HashSet;

fn digit_to_vec(num: u64) -> Vec<u64> {
    fn push_inner(n: u64, digits: &mut Vec<u64>) {
        digits.push(n % 10);

        if n >= 10 {
            push_inner(n/10, digits);
        }
    }
    
    let mut digits: Vec<u64> = vec![];
    push_inner(num, &mut digits);

    digits.into_iter().rev().collect()
}

fn slice_to_int(n_a: &[u64]) -> u64 {
    let mut t = 1;
    let mut n = 0;
    for n_d in n_a.iter().rev() {
        n += *n_d*t;
        t *= 10;
    }

    n
}

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

fn index_permutations(len: usize) -> Vec<Vec<usize>> {
    (1..len).map(|n| (0..len).permutations(n).unique().collect_vec()).collect_vec().concat()
}

fn max_prime_value_family(num: u64) -> Vec<u64> {
    let num_digits = digit_to_vec(num);
    let len = num_digits.len();
    let ind_perm = index_permutations(len);

    let num_digits_replaced = ind_perm.iter()
        .map(|ind| {
            (0..10).map(|d| {
                num_digits.clone().iter()
                    .enumerate()
                    .map(|(i, v)| if ind.contains(&i) { d as u64 } else { *v })
                    .collect_vec()
            })
            .map(|v| slice_to_int(&v[..]))
            .filter(|n| digit_to_vec(*n).len() == len)
            .filter(|n| check_primality(*n))
            .collect_vec()
        })
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

fn main() {
    let mut n = 9;
    let family = 8;

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
            return println!("{:?}", prime_family);
        }
    }
}
