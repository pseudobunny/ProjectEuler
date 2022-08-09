fn is_palindrome(n_str: String) -> bool {
    n_str == n_str.chars().rev().collect::<String>()
}

fn to_base_2(mut n: u32) -> String {
    let mut result = vec![];

    loop {
        let m = n%2;
        n = n/2;

        result.push(std::char::from_digit(m,2).unwrap());
        if n == 0 {
            break;
        }
    }

    result.into_iter().rev().collect()
}

fn is_double_palindrome(n: u32) -> bool {
    is_palindrome(n.to_string()) && is_palindrome(to_base_2(n))
}

fn main() {
    let mut sum = 0;

    for i in 1..1_000_000 {
        if is_double_palindrome(i) {
            sum += i;
        }
    }

    println!("{}", sum)
}
