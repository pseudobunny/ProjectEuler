// THIS CODE WILL NOT RUN
// This is left here because it was a valiant attempt
// Trying to make the Chakravala method work in rust for... very big numbers will probably be the
// death of me.
// If you are looking at this, look for the problem instead solved in julia.

// I may come back to this in the future, as an interesting problem to solve

use num_bigint::ToBigInt;
use std::cmp::Ordering;

// This will be done with the Chakravala method
// Should be fun
fn find_minimum_x(d: u64) -> u64 {
    // Find a solution a^2 - d*b^2 = k for any k found by any means
    // We'll do this by just finding the first a that satisfies a^2 > d*1
    // aka the ceiling of sqrt(d)
    let d_sqrt = (d as f64).sqrt();
    let mut a = d_sqrt.ceil() as u64;
    let mut b = 1;
    let mut k = a*a - d;

    println!("{}", d);
    
    

    let mut conv_list = SqrtConvergentsList::new(d);

    let mut i = 0;
    loop {
        let (_, x, y) = conv_list.get(i);

        let x2 = x*x;
        let dy2 = d*y*y;
        if x2.cmp(&dy2) == Ordering::Greater && x2 - dy2 == 1 {
            return x;
        }

        i += 1;
    }
}

fn next_triple(a: u64, b: u64, k: u64) -> (u64, u64, u64) {

}

fn find_min_m(a: u64, b: u64, k: u64, d: u64, d_sqrt: f64) -> u64 {
    // now we need to choose an m such that (a+b*m)/k
    // that also minimizes |m^2 - d|
    // start at the first multiple of k that is greater than (or equal to) a+b
    let k_mul = ((a+b) as f64 / k as f64).ceil() as u64 * k;
    // just need to check what is left over after a
    let mut k_mul_sub_a = k_mul - a;
    let mut m = 1;
    let mut min_val = 0.0;

    // we want to check if we've crossed the zero point
    let mut crossed = (m*m) as f64 > d_sqrt; 
    loop {
        if k_mul_sub_a % m == 0 {
            
        }
    }
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

fn main() {
    let non_squares: Vec<u64> = (2..=7).filter(|&n| !is_square(n)).collect();

    let max_x = non_squares.iter()
                           .map(|&n| (n, find_minimum_x(n)))
                           .max_by(|a, b| a.1.cmp(&b.1))
                           .unwrap()
                           .0;
    
    println!("{}", max_x);
}
