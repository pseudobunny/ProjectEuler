fn check_if_whole(n: f64) -> bool {
    (n - (n as i64) as f64).abs() < 0.00001
}

fn main() {
    let mut max_sum: u64 = 0;
    let mut max: u64 = 0;

    for n in 15..=1000 {
        let mut sum: u64 = 0;
        
        let f_n = n as f64;
        for x in 1..(n/3) {
            let f_x = x as f64;
            let y = (f_n*0.5 - f_x) / (1.0 - (f_x / f_n));

            if check_if_whole(y) {
                sum += 1;
            }
        }

        if sum > max_sum {
            max_sum = sum;
            max = n;
        }
    }

    println!("{}", max)
}
