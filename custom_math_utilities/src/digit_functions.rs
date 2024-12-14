use crate::num_to_digits;

static FACTORIAL: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];
static SQUARE: [u64; 10] = [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];

pub fn digit_factorial(num: u64) -> u64 {
    digit_function(&FACTORIAL)(num)
}

pub fn digit_square(num: u64) -> u64 {
    digit_function(&SQUARE)(num)
}

fn digit_function(func_array: &[u64; 10]) -> impl Fn(u64) -> u64 + '_ {
    |num| {
        num_to_digits(num)
            .iter()
            .map(|&n| func_array[n as usize])
            .sum()
    }
}
