use num::BigUint;

pub struct ConvergentsList {
    pub a: Vec<BigUint>,
    pub h: Vec<BigUint>,
    pub k: Vec<BigUint>,
    pattern: fn(usize) -> BigUint,
}

impl ConvergentsList {
    pub fn new(a0: u64, a1: u64, pattern: fn(usize) -> BigUint) -> ConvergentsList {
        let a0 = BigUint::from(a0);
        let a1: BigUint = BigUint::from(a1);

        ConvergentsList {
            a: vec![a0.clone(), a1.clone()],
            h: vec![a0.clone(), a1.clone() * a0 + 1_u32],
            k: vec![BigUint::from(1_u32), a1],
            pattern,
        }
    }

    fn extend(&mut self) {
        let a_n = (self.pattern)(self.a.len());
        self.a.push(a_n.clone());

        let hk_len = self.h.len();

        self.h
            .push(a_n.clone() * self.h[hk_len - 1].clone() + self.h[hk_len - 2].clone());
        self.k
            .push(a_n * self.k[hk_len - 1].clone() + self.k[hk_len - 2].clone());
    }

    pub fn get(&mut self, i: usize) -> (BigUint, BigUint, BigUint) {
        while i >= self.a.len() {
            self.extend();
        }
        (self.a[i].clone(), self.h[i].clone(), self.k[i].clone())
    }
}
