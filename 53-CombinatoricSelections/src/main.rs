use memoize::memoize;
use num_bigint::{ ToBigUint, BigUint };
use num_traits::{Zero, One};

#[memoize]
fn factorial(n: BigUint) -> BigUint {
    if n == Zero::zero() {
        One::one()
    } else {
        n.clone() * factorial(n- 1_u64)
    }
}

fn combination(n: BigUint, r: BigUint) -> BigUint {
    factorial(n.clone())/(factorial(r.clone()) * factorial(n-r))
}

fn main() {
    let combinatorics_vec = (1..=100)
        .map(|n| (n.to_biguint().unwrap(), (1..=n).map(|r| (r as u64).to_biguint().unwrap()).collect::<Vec<BigUint>>()))
        .map(|(n,rv)| rv.iter().map(|r| combination(n.clone(), r.clone())).collect::<Vec<BigUint>>())
        .collect::<Vec<Vec<BigUint>>>()
        .concat();

    let combinatorics = combinatorics_vec.iter()
        .filter(|c| **c  > 1_000_000.to_biguint().unwrap());

    println!("{}", combinatorics.count())
}
