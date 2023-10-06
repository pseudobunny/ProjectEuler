use crate::num_to_digits;

static FACTORIAL: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

pub fn digit_factorial(num: u64) -> u64 {
    num_to_digits(num)
        .iter()
        .map(|&n| FACTORIAL[n as usize])
        .sum()
}
