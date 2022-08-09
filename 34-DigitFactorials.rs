static FACTORIAL: [u64; 10] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5_040,
    40_320,
    362_880
];

fn sum_factorial_digits(num: u64) -> u64 {
    fn sum_inner(n: u64, sum: &mut u64) {
        *sum += FACTORIAL[(n % 10) as usize];
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

    for i in 3..1_000_000 {
        if sum_factorial_digits(i) == i {
            sum += i;
        }
    }

    println!("{}", sum)
}
