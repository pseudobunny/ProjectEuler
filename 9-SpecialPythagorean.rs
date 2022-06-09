fn check_if_whole(n: f32) -> bool {
    (n - n.trunc()).abs() < 0.00001_f32
}

fn main() {
    let mut y : f32;
    for x in 1..500 {
        y = (1000.0 - 2.0*x as f32) / (2.0 - (1.0/500.0)*x as f32);
        if check_if_whole(y) {
            println!("x: {}, y: {}, z: {}", x, y, ((x*x) as f32 + y*y).sqrt());
            break;
        }
    }
}