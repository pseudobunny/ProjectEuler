fn generate_penta_num(n: u64) -> u64 {
     (n * (3*n - 1)) / 2
} 

struct PentaNumberList {
    list: Vec<u64>
}

impl PentaNumberList {
    fn new() -> PentaNumberList {
        PentaNumberList{ list: vec![1, 5, 12, 22, 35, 51, 70, 92, 117, 145] }
    }

    fn extend(&mut self) {
        let new_tri_num = generate_penta_num(self.list.len() as u64 + 1);
        self.list.push(new_tri_num);
    }

    fn get(&mut self, i: usize) -> u64 {
        while i > self.list.len() {
            self.extend();
        }

        self.list[i-1]
    }

    fn is_in(&mut self, n: u64) -> bool {
        while self.list.last().unwrap() < &n {
           self.extend(); 
        }

        self.list.contains(&n)
    }
} 

fn main () {
    let mut found = false;
    let mut min_d = 0;
    let mut penta_list = PentaNumberList::new();

    let mut i = 1;
    let view_range = 1000;
    while !found {
        let a = penta_list.get(i);
        for j in 1..=view_range {
            let b = penta_list.get(i+j);
            
            if penta_list.is_in(a+b) {
                if penta_list.is_in(b+b+a) {
                    min_d = a;
                    found = true;
                    break;
                }
            }
            if penta_list.is_in(b-a) {
                if penta_list.is_in(b+b-a) {
                    min_d = a;
                    found = true;
                    break;
                }
            }
        }

        i += 1;
    }

    println!("{}", min_d);
}
