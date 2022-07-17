use std::collections::{
    HashMap,
    HashSet,
};

fn d(n: u64) -> u64 {
    let max: u64 = (n as f32).sqrt().ceil() as u64;
    let mut sum: u64 = 1;
    let mut i: u64 = 2;
    while i < max {
        if n%i == 0 {
            sum += i + (n/i);
        }
        i += 1;
    }
    if i*i == n {
        sum += i;
    }

    sum
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

    println!("{}", amicable_numbers.iter().sum::<u64>())
}