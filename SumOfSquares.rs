fn sum_of_squares(start: u32, end: u32) -> u32 {
    let mut sum = 0;
    for i in start..=end {
        sum += i*i; 
    }
    sum
}

fn square_of_sum(start: u32, end: u32) -> u32 {
    let mut sum = 0;
    for i in start..=end {
        sum += i; 
    }
    sum*sum
}

fn main() {
    println!("{}", square_of_sum(1, 100) - sum_of_squares(1,100))
}