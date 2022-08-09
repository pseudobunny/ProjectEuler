fn sum_fifth_digits(num: u64) -> u64 {
    fn sum_inner(n: u64, sum: &mut u64) {
        *sum += (n % 10).pow(5);
        if n >= 10 {
            sum_inner(n/10, sum);
        }
    }

    let mut sum = 0;
    sum_inner(num, &mut sum);

    sum
}

fn main() {
    let mut sum = 0;
    for i in 2..1_000_000 {
        if i == sum_fifth_digits(i) {
            sum += i;
        }
    }

    println!("{}", sum)
}
