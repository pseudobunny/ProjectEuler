use std::collections::{
    HashMap,
    HashSet,
};

fn d(n: u64) -> u64 {
    let mut max : u64 = n/2;
    let mut divisor_sum: u64 = 1;
    let mut i : u64 = 2;
    loop {
        if i >= max {
            break;
        }

        if n % i == 0 {
            max = n / i;
            divisor_sum += i + max;
        }

        i += 1;
    }

    divisor_sum
}


fn main() {
    let mut d_results = HashMap::new();
    let mut amicable_numbers = HashSet::new();

    let mut d_result;
    for n in 2..10000 {
        d_result = d(n);
        d_results.insert(n,d_result);
        if n != d_result && d_results.contains_key(&d_result) && d_results[&d_result] == n {
            amicable_numbers.insert(d_result);
            amicable_numbers.insert(d_results[&d_result]);
        }
    }

    for an in &amicable_numbers {
        println!("{}", an);
    }

    println!("{}", amicable_numbers.iter().sum::<u64>())
}