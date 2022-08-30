use itertools::Itertools;
use std::cmp;

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

fn slice_to_int(n_a: &[&u64]) -> u64 {
    let mut t = 1;
    let mut n = 0;
    for n_d in n_a.iter().rev() {
        n += *n_d*t;
        t *= 10;
    }

    n
}

fn main() {
    let mut max = 0;
    let mut digits = vec![];

    for i in 1..=9 {
        digits.push(i);
        
        for perm in digits.iter().permutations(digits.len()).unique() {
            let num = slice_to_int(&perm[..]);
            if check_primality(num) {
                max = cmp::max(max, num);
            }
        }
    }

    println!("{}", max)
}
