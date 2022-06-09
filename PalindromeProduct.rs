fn check_palindrome(x: u32) -> bool {
    let x_str = x.to_string();
    let n = x_str.len() / 2;

    x_str.bytes().take(n).eq(x_str.bytes().rev().take(n))
}

fn main() {
    let mut max_palindrome = 0;

    let max_num = 1_000_000;
    let max_factor = 1000;
    let min_factor = 100;
    for i in 1..max_num {
        let curr_num = max_num - i;
        
        if check_palindrome(curr_num) {
            for j in 1..(max_factor - min_factor) {
                let curr_factor = max_factor - j;
                let second_factor = curr_num / curr_factor;
                
                if (curr_num % curr_factor == 0) && (second_factor < max_factor && second_factor >= min_factor) {
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

    println!("{}", max_palindrome)
}