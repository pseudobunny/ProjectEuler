use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

fn main() {
    let max = format!("{}{}","1","0".repeat(999)).parse::<BigUint>().expect("error parsing big int");

    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    let mut i = 1;
    loop {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
        i += 1;

        if f1 >= max {
            break;
        }
    }

    println!("{}", i)
}
