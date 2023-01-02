use num::BigUint;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn sum_large_nums_first_ten(filename: &str) -> String {
    let large_sum: BigUint = lines_from_file(filename)
        .iter()
        .map(|l| l.parse::<BigUint>().unwrap())
        .sum();

    large_sum.to_string()[0..10].to_string()
}

fn main() {
    println!("{}", sum_large_nums_first_ten("src/fiftydigitnums.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(
            sum_large_nums_first_ten("src/fiftydigitnums.txt"),
            "5537376230"
        )
    }
}
