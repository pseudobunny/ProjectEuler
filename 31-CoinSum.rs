fn main() {
    let total = 200;
    let sizes = vec![1,2,5,10,20,50,100,200];
    let mut configurations = vec![0; total+1];
    configurations[0] = 1;

    for i in 0..8 {
        for j in sizes[i]..=total {
            configurations[j] += configurations[j - sizes[i]];
        }
    }

    println!("{}", configurations[total])
}
