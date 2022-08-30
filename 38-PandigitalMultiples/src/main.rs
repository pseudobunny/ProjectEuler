use std::collections::HashSet;
use std::cmp;

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

fn is_pandigital(n_a: &Vec<u64>) -> bool {
    let mut set = HashSet::new();

    for n in n_a.iter() {
        set.insert(n);
    }

    set.len() == n_a.len()
}

fn main() {
    let mut n: u64 = 192;
    let mut max: u64 = 0;

    while n < 100_000 {
        let mut collect: Vec<u64> = vec![];
        let mut m: u64 = 1;
        loop {
            let mut a = digit_to_vec(n*m);
            collect.append(&mut a);

            if collect.len() > 8 {
                break;
            }

            m += 1;
        }
        
        n += 1;
        
        if collect.contains(&0) {
            continue;
        }

        if is_pandigital(&collect) {
            max = cmp::max(max, slice_to_int(&collect[..]))
        }
    }

    println!("{}", max)
}
