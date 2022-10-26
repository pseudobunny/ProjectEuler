fn generate_triangle_num(n: u64) -> u64 {
     (n * (n+1)) / 2
}

fn generate_penta_num(n: u64) -> u64 {
     (n * (3*n - 1)) / 2
} 

fn generate_hexa_num(n: u64) -> u64 {
    n * (2*n - 1)
}

struct GeneratedFuncList {
    generator_func: fn(u64) -> u64,
    list: Vec<u64>
}

impl GeneratedFuncList {
    fn new(func: fn(u64) -> u64) -> GeneratedFuncList {
        GeneratedFuncList{ generator_func: func, list: vec![1] }
    }

    fn extend(&mut self) {
        let new_tri_num = (self.generator_func)(self.list.len() as u64 + 1);
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
    let mut tri_list  = GeneratedFuncList::new(generate_triangle_num);
    let mut penta_list = GeneratedFuncList::new(generate_penta_num);
    let mut hexa_list = GeneratedFuncList::new(generate_hexa_num);

    let mut i = 286;
    loop {
        let a = tri_list.get(i);

        if penta_list.is_in(a) && hexa_list.is_in(a) {
            println!("{}", a);
            break;
        }

        i += 1;
    }
}
