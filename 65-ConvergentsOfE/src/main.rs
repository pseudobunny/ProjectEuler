use num_bigint::{ToBigInt, BigInt};
use num_traits::{Zero, One};

struct ConvergentsList {
    a: Vec<BigInt>,
    // special case - we know the a generation function
    //curr_x: f64, // The previous "x" in the calculation
    h: Vec<BigInt>,
    // Only need the numerator
    //k: Vec<u64>,
}

impl ConvergentsList {
    fn new() -> ConvergentsList {
        //let mut curr_x = x;
        let a0 = 2.to_bigint().unwrap(); //curr_x.trunc() as u64;
        //curr_x = 1.0 / (curr_x - a0 as f64);
        let a1: BigInt = One::one(); //curr_x.trunc() as u64;

        ConvergentsList {
            a: vec![a0.clone(), a1.clone()],
            //curr_x,
            h: vec![a0.clone(), a1*a0 + 1],
            //k: vec![1, a1],
        } 
    }

    fn extend(&mut self) {
        //self.curr_x = 1.0 / (self.curr_x - *self.a.last().unwrap() as f64);
        let len_a: BigInt = self.a.len().to_bigint().unwrap();

        let curr_a: BigInt = if (len_a.clone()+2) % 3 == One::one() {
            (len_a+2)/3 * 2
        } else {
            One::one()
        };

        self.a.push(curr_a.clone());
        
        let hk_len = self.h.len();

        self.h.push(curr_a*self.h[hk_len-1].clone() + self.h[hk_len-2].clone());
        //self.k.push(curr_a*self.k[hk_len-1] + self.k[hk_len-2]);
    }

    fn get(&mut self, i: usize) -> (BigInt, BigInt) {
        while i >= self.a.len() {
            self.extend();
        }
        (self.a[i].clone(), self.h[i].clone())
        //(self.a[i], self.h[i], self.k[i]) 
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
