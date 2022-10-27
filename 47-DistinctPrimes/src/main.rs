use std::collections::HashSet;

fn prime_factorization_set(n: u64) -> HashSet<u64> {
    let mut prime_set = HashSet::new();
    let mut n = n;

    while n % 2  == 0 {
        prime_set.insert(2);
        n /= 2;
    }

    let sqrt_n = ((n as f64).sqrt()) as u64;
    for i in (3..=sqrt_n).step_by(2) {
        while n % i == 0 {
            prime_set.insert(i);
            n /= i;
        }
    }

    if n > 2 {
        prime_set.insert(n);
    }

    prime_set
}

fn main() {
    let mut consecutive = vec![];

    let target = 4;
    let mut i = 2;
    while consecutive.len() < target {
        if prime_factorization_set(i).len() == target {
            consecutive.push(i);
        } else {
            consecutive.clear()
        }

        i += 1;
    }

    println!("{:?}", consecutive)
}
