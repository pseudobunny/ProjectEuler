fn mul3or5(val: i32) -> bool {
    (val % 3 == 0) || (val % 5 == 0)
}

fn main() {
    let mut x : i32 = 0;
    for i in 3..1000 {
        if mul3or5(i) {
            x += i;
        }
    }
    println!("{}", x);
}