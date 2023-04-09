pub struct GeneratedFuncList {
    generator_func: fn(u64) -> u64,
    list: Vec<u64>,
}

impl GeneratedFuncList {
    pub fn new(func: fn(u64) -> u64) -> GeneratedFuncList {
        GeneratedFuncList {
            generator_func: func,
            list: vec![1],
        }
    }

    pub fn extend(&mut self) {
        let new_tri_num = (self.generator_func)(self.list.len() as u64 + 1);
        self.list.push(new_tri_num);
    }

    pub fn extend_until(&mut self, n: u64) {
        while self.list.last().unwrap() < &n {
            self.extend();
        }
    }

    pub fn get(&mut self, i: usize) -> u64 {
        while i > self.list.len() {
            self.extend();
        }

        self.list[i - 1]
    }

    pub fn is_in(&mut self, n: u64) -> bool {
        self.extend_until(n);

        self.list.contains(&n)
    }

    pub fn values_between(&mut self, min: u64, max: u64) -> Vec<u64> {
        self.extend_until(max);

        self.list
            .iter()
            .filter(|&&n| n > min && n < max)
            .copied()
            .collect()
    }
}

fn generate_triangle_num(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

fn generate_square_num(n: u64) -> u64 {
    n * n
}

fn generate_penta_num(n: u64) -> u64 {
    (n * (3 * n - 1)) / 2
}

fn generate_hexa_num(n: u64) -> u64 {
    n * (2 * n - 1)
}

fn generate_hepta_num(n: u64) -> u64 {
    n * (5 * n - 3) / 2
}

fn generate_octa_num(n: u64) -> u64 {
    n * (3 * n - 2)
}

pub fn triangle_number_list() -> GeneratedFuncList {
    GeneratedFuncList::new(generate_triangle_num)
}

pub fn square_number_list() -> GeneratedFuncList {
    GeneratedFuncList::new(generate_square_num)
}

pub fn penta_number_list() -> GeneratedFuncList {
    GeneratedFuncList::new(generate_penta_num)
}

pub fn hexa_number_list() -> GeneratedFuncList {
    GeneratedFuncList::new(generate_hexa_num)
}

pub fn hepta_number_list() -> GeneratedFuncList {
    GeneratedFuncList::new(generate_hepta_num)
}

pub fn octa_number_list() -> GeneratedFuncList {
    GeneratedFuncList::new(generate_octa_num)
}
