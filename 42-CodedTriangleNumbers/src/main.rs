use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    
    let buf = BufReader::new(file);
    
    buf.lines()
       .map(|l| l.expect("Could not parse line").trim().to_string())
       .collect()
}

fn word_line_to_names(line: &str) -> Vec<String> {
    line.split(',').map(|s| s.trim_matches('"').to_string()).collect()
}

fn convert_word_to_sum(word: &str) -> u8 {
    word.chars()
        .into_iter()
        .map(|c| c as u8)
        .map(|c| (c - 64u8))
        .sum()
}

fn generate_triangle_num(n: i32) -> i32 {
     (n * (n+1)) / 2
} 

struct TriangleNumberList {
    list: Vec<i32>
}

impl TriangleNumberList {
    fn new() -> TriangleNumberList {
        TriangleNumberList{ list: vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55] }
    }

    fn extend(&mut self) {
        let new_tri_num = generate_triangle_num(self.list.len() as i32 + 1);
        self.list.push(new_tri_num);
    }

    fn is_in(&mut self, n: i32) -> bool {
        while self.list.last().unwrap() < &n {
           self.extend(); 
        }

        self.list.contains(&n)
    }
}

fn main() {
    let input_lines = lines_from_file("p042_words.txt");
    
    let mut tri_list = TriangleNumberList::new(); 
    let input_sums_that_are_tri = word_line_to_names(&input_lines[0])
        .into_iter()             
        .map(|w| convert_word_to_sum(&w) as i32) // convert to sum
        .filter(|s| tri_list.is_in(*s)); // filter out words that aren't triangle numbers    

    println!("{}", input_sums_that_are_tri.count())
}
