use std::collections::HashSet;

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
    let mut abundant_nums: Vec<u64> = vec![];
    let mut abundant_sums = HashSet::new();

    println!("{}", d(12));

    // abundant num generation
    println!("Generating abundant nums");
    for n in 12..28124 {
        if n < d(n) {
            abundant_nums.push(n);
        }
    }

    // abundant sum generation
    println!("Generating abundant sums");
    let mut sum: u64;
    for i in 0..abundant_nums.len() {
        for j in i..abundant_nums.len() {
            sum = abundant_nums[i] + abundant_nums[j];
            
            if sum > 28123 {
                break;
            }
            
            abundant_sums.insert(sum);
        }
    }

    // non-abundant sum
    println!("Checking for non abundant nums");
    sum = 0;
    for n in 1..28124 {
        if !abundant_sums.contains(&n) {
            sum += n;
        }
    }

    println!("{}", sum)
}