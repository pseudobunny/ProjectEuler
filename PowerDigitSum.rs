use std::env;

fn add_large_num_ordered(ln1: &Vec<u64>, ln2: &Vec<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let max_part_size: u64 = 10000000000;

    let mut rollover: u64 = 0;
    for i in 0..ln1.len() {
        let mut part_result: u64 = ln1[i] + ln2[i] + rollover;
        
        if part_result > max_part_size {
            rollover = part_result / max_part_size;
            part_result -= rollover * max_part_size;
        }
        else {
            rollover = 0;
        }

        result.push(part_result);
    }

    if ln2.len() > ln1.len() {
        for i in ln1.len()..ln2.len() {
            let mut part_result: u64 = ln2[i] + rollover;

            if part_result > max_part_size {
                rollover = part_result / max_part_size;
                part_result -= rollover * max_part_size;
            }
            else {
                rollover = 0;
            }

            result.push(part_result);
        }
    }
    else if rollover > 0 {
        result.push(rollover);
    }

    result
}

fn double_big_num(bn: Vec<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut rollover: Vec<u64> = vec![0];

    let max_n_size: u64 = 10000000000;

    for n in bn.iter() {
        let mut curr_result = n * 2;
        let mut curr_rollover = 0;
        if curr_result > max_n_size {
            curr_rollover = curr_result / max_n_size;
            curr_result -= curr_rollover*max_n_size;
        }
        
        result.push(curr_result);
        rollover.push(curr_rollover);
    }

    if rollover[rollover.len()-1] == 0 {
        rollover.pop();
    }

    add_large_num_ordered(&result, &rollover)
}

fn print_large_num(ln: Vec<u64>) -> String {
    let mut out_str: String = "".to_string();

    for i in 0..ln.len() {
        let mut n_str: String = ln[i].to_string();

        let size_diff = 10 - n_str.len();
        if (size_diff > 0) && (i != ln.len()-1) {
            n_str = "0".repeat(size_diff) + &n_str;
        }

        out_str = n_str + &out_str;
    }

    out_str
}

fn sum_digits(num_str: String) -> u32 {
    num_str.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let power = (&args[1]).parse::<u32>().expect("Improper power in arguments");

    let mut power_result: Vec<u64> = vec![2];
    //let power = 15;

    for _ in 1..power {
        power_result = double_big_num(power_result);
    }

    println!("{}", sum_digits(print_large_num(power_result)));
}