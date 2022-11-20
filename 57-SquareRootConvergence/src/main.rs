use num::BigInt;

struct ConvergentsList {
    a: Vec<BigInt>,
    h: Vec<BigInt>,
    k: Vec<BigInt>
}

impl ConvergentsList {
    fn new() -> ConvergentsList {
        let a0 = BigInt::from(1);
        let a1: BigInt = BigInt::from(2); 

        ConvergentsList {
            a: vec![a0.clone(), a1.clone()],
            h: vec![a0.clone(), a1.clone()*a0 + 1],
            k: vec![BigInt::from(1), a1]
        } 
    }

    fn extend(&mut self) {
        self.a.push(BigInt::from(2));
        
        let hk_len = self.h.len();

        self.h.push(BigInt::from(2)*self.h[hk_len-1].clone() + self.h[hk_len-2].clone());
        self.k.push(BigInt::from(2)*self.k[hk_len-1].clone() + self.k[hk_len-2].clone());
    }

    fn get(&mut self, i: usize) -> (BigInt, BigInt, BigInt) {
        while i >= self.a.len() {
            self.extend();
        }
        (self.a[i].clone(), self.h[i].clone(), self.k[i].clone())
    }
}

fn digit_len(num: BigInt) -> u64 {
    fn sum_inner(n: BigInt, sum: &mut u64) {
        *sum += 1;

        if n >= BigInt::from(10_u64) {
            sum_inner(n/10_u64, sum);
        }
    }
    
    let mut sum = 0;
    sum_inner(num, &mut sum);

    sum
}

fn main() {
    let mut conv_list = ConvergentsList::new();
    conv_list.get(1000);
    
    let num_greater_than_den = conv_list.h.iter().zip(conv_list.k.iter())
        .map(|(h,k)| (digit_len(h.clone()), digit_len(k.clone())))
        .filter(|(hl, kl)| hl > kl)
        .count();

    println!("{}", num_greater_than_den)
}
