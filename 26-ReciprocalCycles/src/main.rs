use num_bigint::{ToBigUint};
use num_traits::One;

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

    return true;
}

fn get_order(p: u64) -> u64 {
    let mut k = 1;
    let big_uint_ten = 10.to_biguint().expect("");
    let mut curr_mult = 10.to_biguint().expect("");

    let mut r;
    loop {
        r = &curr_mult % p;
        
        if r == One::one() {
            break;
        }

        curr_mult *= &big_uint_ten;
        k += 1;
    }

    k
}

fn main() {
    let mut max_order = 0;
    let mut max_d = 0;

    let mut order: u64;
    for d in 95..1000 {
        if check_primality(d) {
            order = get_order(d);
            if order > max_order {
                max_order = order;
                max_d = d;
            }
        }
    }

    println!("{}", max_d)
}