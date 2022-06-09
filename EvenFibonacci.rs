fn update_euler(x: (u32, u32)) -> (u32, u32) {
    (x.1, x.0 + x.1)
}

fn main() {
    let mut fib_pair : (u32, u32) = (1, 2);
    let mut fib_sum : u32 = 2;

    while fib_pair.1 < 4_000_000 {
        fib_pair = update_euler(fib_pair);
        if fib_pair.1 % 2 == 0 {
            fib_sum += fib_pair.1;
        }
    }

    println!("{}", fib_sum);
}