fn check_palindrome(x: u32) -> bool {
    let x_str = x.to_string();
    let n = x_str.len() / 2;

    x_str.bytes().take(n).eq(x_str.bytes().rev().take(n))
}

fn max_palindrome(min: u32, max: u32) -> u32 {
    let mut max_palindrome = 0;

    let max_num = max * max;
    for i in 1..max_num {
        let curr_num = max_num - i;

        if check_palindrome(curr_num) {
            for j in 1..(max - min) {
                let curr_factor = max - j;
                let second_factor = curr_num / curr_factor;

                if (curr_num % curr_factor == 0) && (second_factor < max && second_factor >= min) {
                    max_palindrome = curr_num;
                    println!("{}", curr_factor);
                    println!("{}", second_factor);
                    break;
                }
            }
        }

        if max_palindrome > 0 {
            break;
        }
    }

    max_palindrome
}

fn main() {
    println!("{}", max_palindrome(100, 1000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(max_palindrome(10, 100), 9009)
    }

    #[test]
    fn q_case() {
        assert_eq!(max_palindrome(100, 1000), 906609)
    }
}
