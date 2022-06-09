fn check_if_multiple(x: u64, divisors: &[u64]) -> bool {
    let mut all_divisors = true;
    
    for i in divisors.iter() {
        if x % i != 0 {
            all_divisors = false;
            break;
        }
    }

    all_divisors
}

fn main() {
    let factors = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];

    let mut smallest_multiple = 20;
    while !check_if_multiple(smallest_multiple, &factors) {
        smallest_multiple += 1;
    }

    println!("{}", smallest_multiple)
}