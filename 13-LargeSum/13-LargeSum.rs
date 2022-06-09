use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

// FUNCTIONS TO READ FILE TO VECTOR OF LARGE NUMBERS
fn to_large_num(num_str: &str) -> Vec<u64> {
    let mut large_num: Vec<u64> = vec![];
    let max_part_len = 10;

    for i in 0..5 {
        let start_ind = i*max_part_len;
        let num_part = &num_str[start_ind..start_ind+max_part_len];

        large_num.insert(0,num_part.parse::<u64>().expect("Could not parse num"));
    }

    large_num
}

fn lines_to_large_nums(lines: Vec<String>) -> Vec<Vec<u64>> {
    let mut large_nums: Vec<Vec<u64>> = vec![];

    for line in lines.iter() {
        large_nums.push(to_large_num(line.trim()));
    }

    large_nums
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    
    let buf = BufReader::new(file);
    
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

// COPY LARGE NUM
fn copy_large_num(ln: &Vec<u64>) -> Vec<u64> {
    let mut new_ln: Vec<u64> = vec![];

    for l in ln.iter() {
        new_ln.push(*l);
    }

    new_ln
}

// FUNCTIONS TO ADD LARGE NUMBERS
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

// PRINT LARGE NUM
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

fn main() {
    let large_nums = lines_to_large_nums(lines_from_file("fiftydigitnums.txt"));

    let mut current_num: Vec<u64> = copy_large_num(&large_nums[0]);
    for i in 1..100 {
        current_num = add_large_num_ordered(&large_nums[i], &current_num);
    }

    println!("{}", print_large_num(current_num))
}