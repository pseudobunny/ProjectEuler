use num_bigint::{ ToBigInt, BigInt};
use num::Signed;
use num_traits::{Zero, One};
use num_iter::range;

fn chakravala(a: BigInt, b: BigInt, k: BigInt, d: BigInt) -> (BigInt, BigInt) {
    let (new_a, new_b, new_k) = next_triple
        (
            a.clone(),
            b.clone(), 
            k.clone(), 
            find_min_m(a,b,k,d.clone()), 
            d.clone()
        );

    if new_k != One::one() {
        return chakravala(new_a, new_b, new_k, d); 
    }
    
    (new_a, new_b)
}

fn next_triple(a: BigInt, b: BigInt, k: BigInt, m: BigInt, d: BigInt) -> (BigInt, BigInt, BigInt) {
    let new_a = (a.clone()*m.clone() + d.clone()*b.clone()) / k.abs();
    let new_b = (a + b*m.clone()) / k.abs();
    let new_k = (m.clone()*m - d) / k;

    (new_a, new_b, new_k)
}

fn find_min_m(a: BigInt, b: BigInt, k: BigInt, d: BigInt) -> BigInt {
    // Find the starting m
    let mut start_m = Zero::zero();
    for i in range(Zero::zero(),k.abs()) {
        if (a.clone() + b.clone()*i.clone()) % k.clone() == Zero::zero() {
            start_m = i.clone();
        }        
    }

    // Define search space
    let m_to_search = (0..10).map(|i| i.to_bigint().unwrap())
                             .map(|i| start_m.clone() + (i*k.clone()).abs());
    
    // Find the minimum
    let mut least_m = start_m.clone();
    for m in m_to_search {
        if (m.clone()*m.clone() - d.clone()).abs() < (least_m.clone()*least_m.clone() - d.clone()).abs() {
            least_m = m;
        }
    }

    least_m
}

fn is_square(n: u64) -> bool {
    let n_sqrt = (n as f64).sqrt();
    
    (n_sqrt - n_sqrt.trunc()).abs() < 5e-5
}

// Attempting to find the fundamental solution killed the program
// This was... not the correct way to efficiently do this.
// But IT TOOK ME TIME TO UNDERSTAND so it stays
//struct SqrtConvergentsList {
//    a: Vec<u64>,
//    curr_x: f64, // The previous "x" in the calculation
//    h: Vec<u64>,
//    k: Vec<u64>,
//}

//impl SqrtConvergentsList {
//    fn new(x: u64) -> SqrtConvergentsList {
//        let mut curr_x = (x as f64).sqrt();
//        let a0 = curr_x.trunc() as u64;
//        curr_x = 1.0 / (curr_x - a0 as f64);
//        let a1 = curr_x.trunc() as u64;
//
//        SqrtConvergentsList {
//            a: vec![a0, a1],
//            curr_x,
//            h: vec![a0, a1*a0 + 1],
//            k: vec![1, a1],
//        } 
//    }

//    fn extend(&mut self) {
//        self.curr_x = 1.0 / (self.curr_x - *self.a.last().unwrap() as f64);
//        self.a.push(self.curr_x.trunc() as u64);
//        
//        let curr_a = *self.a.last().unwrap();
//        let hk_len = self.h.len();
//
//        self.h.push(curr_a*self.h[hk_len-1] + self.h[hk_len-2]);
//        self.k.push(curr_a*self.k[hk_len-1] + self.k[hk_len-2]);
//    }

//    fn get(&mut self, i: usize) -> (u64, u64, u64) {
//        while i >= self.a.len() {
//            self.extend();
//        }

//        (self.a[i], self.h[i], self.k[i]) 
//    }
//}

// because rust is weird
fn big_int_one() -> BigInt {
    One::one()
}

// Just to make the call in the map nicer
fn chakravala_starter(n: BigInt) -> (BigInt, BigInt) {
    chakravala(big_int_one(), big_int_one(), big_int_one() - n.clone(), n)
}

fn main() {
    let max = 1000;
    let non_squares: Vec<u64> = (2..=max).filter(|&n| !is_square(n)).collect();

    let max_x = non_squares.iter()
                           .map(|i| i.to_bigint().unwrap())
                           .map(|n| (n.clone(), chakravala_starter(n).0))
                           .max_by(|a, b| a.1.cmp(&b.1))
                           .unwrap()
                           .0;
    
    println!("{}", max_x);
}
