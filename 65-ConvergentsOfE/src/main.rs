use num_bigint::{ToBigInt, BigInt};
use num_traits::{Zero, One};

struct ConvergentsList {
    a: Vec<BigInt>,
    h: Vec<BigInt>,
}

impl ConvergentsList {
    fn new() -> ConvergentsList {
        let a0 = 2.to_bigint().unwrap();
        let a1: BigInt = One::one(); 

        ConvergentsList {
            a: vec![a0.clone(), a1.clone()],
            h: vec![a0.clone(), a1*a0 + 1],
        } 
    }

    fn extend(&mut self) {
        let len_a: BigInt = self.a.len().to_bigint().unwrap();

        let curr_a: BigInt = if (len_a.clone()+2) % 3 == One::one() {
            (len_a+2)/3 * 2
        } else {
            One::one()
        };

        self.a.push(curr_a.clone());
        
        let hk_len = self.h.len();

        self.h.push(curr_a*self.h[hk_len-1].clone() + self.h[hk_len-2].clone());
    }

    fn get(&mut self, i: usize) -> (BigInt, BigInt) {
        while i >= self.a.len() {
            self.extend();
        }
        (self.a[i].clone(), self.h[i].clone())
    }
}

fn sum_of_digits(mut n: BigInt) -> BigInt {
    let mut sum = Zero::zero();
    
    while n != Zero::zero() {
        sum += n.clone()%10;
        n /= 10;
    }

    sum
}

fn main() {
    let mut e_conv = ConvergentsList::new();
    println!("{}", sum_of_digits(e_conv.get(99).1))
}
