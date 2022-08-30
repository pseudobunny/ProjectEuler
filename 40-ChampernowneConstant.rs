// This is going to be a very brute force solution...
// There's probably a smart way to count / figure this out, but the brute
// force is so easy with the tools I've built for previous problems.

fn digit_to_vec(num: u64) -> Vec<u64> {
    fn push_inner(n: u64, digits: &mut Vec<u64>) {
        digits.push(n % 10);

        if n >= 10 {
            push_inner(n/10, digits);
        }
    }
    
    let mut digits: Vec<u64> = vec![];
    push_inner(num, &mut digits);

    digits.into_iter().rev().collect()
}

fn main() {
    let mut collect: Vec<u64> = vec![];

    let mut n = 1;
    while collect.len() < 1_000_000 {
        let mut d_vec = digit_to_vec(n);
        collect.append(&mut d_vec);
        n += 1;
    }

    let mut p = 1;
    let mut t = 1;
    for _ in 0..7 {
        p *= collect[t-1];
        t *= 10;
    }

    println!("{}", p)
}
