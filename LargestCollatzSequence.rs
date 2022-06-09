fn collatz_sequence_inner(start_num: u64, step: u32) -> u32 {
    let next_step = step + 1;
    let next_num: u64;

    if start_num%2 == 0 {
        next_num = start_num / 2;
    } else {
        next_num = start_num*3 + 1; 
    }

    if next_num == 1 {
        return next_step;
    } else {
        return collatz_sequence_inner(next_num, next_step);
    }
}

fn collatz_sequence(start_num: u64) -> u32 {
    return collatz_sequence_inner(start_num, 0);
}

fn main() {
    let mut max_steps: u32 = 0;
    let mut max_n: u64 = 0;

    for i in 1..1000001 {
        let current_steps = collatz_sequence(i);
        if max_steps < current_steps {
            max_steps = current_steps;
            max_n = i;
        }
    }

    println!("{}", max_n);
}