fn main() {
    let mut sum = 1;
    let mut prev_num = 1;
    for i in 1..=500 {
        let add = 2*i;
        for _ in 1..=4 {
            prev_num += add;
            sum += prev_num;
        }
    }

    println!("{}", sum)
}