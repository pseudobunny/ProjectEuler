use itertools::Itertools;

fn can_divide(a: u64, b: u64) -> bool {
    a % b == 0
}

fn check_substring_divisibility(num: &str) -> bool {
   let divisors = vec![2,3,5,7,11,13,17];
   let substrings = num.chars()
                       .collect::<Vec<char>>();
   let substrings_nums = substrings.windows(3)
                                   .skip(1)
                                   .map(|vc| vc.iter().collect::<String>())
                                   .map(|s| s.parse::<u64>().expect("Could not parse num"));
   
   substrings_nums.zip(divisors)
                  .all(|(s_n, d)| can_divide(s_n, d))
}

fn main() {
    let digits = vec!["0","1","2","3","4","5","6","7","8","9"];
    let sum: u64 = digits.iter()
                         .copied()
                         .permutations(digits.len())
                         .unique()
                         .map(|p| p.join(""))
                         .filter(|p| check_substring_divisibility(p))
                         .map(|p| p.parse::<u64>().expect("Could not parse num"))
                         .sum();

    println!("{}", sum);
}
