use custom_math_utilities::{digits_to_num, num_to_digits};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn lines_to_uints(filename: &str) -> Vec<u64> {
    lines_from_file(filename)
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

fn analysis(filename: &str) -> HashMap<u64, HashSet<u64>> {
    let mut map = HashMap::new();

    let lines = lines_to_uints(filename);
    
    lines
        .iter()
        .flat_map(|&n| num_to_digits(n))
        .dedup_by(|d1, d2| d1 == d2)
        .for_each(|d| {
            map.insert(d, HashSet::new());
        });

    lines
        .iter()
        .flat_map(|&n| {
            num_to_digits(n)
                .windows(2)
                .map(|v| Vec::from(v))
                .collect::<Vec<_>>()
        })
        .for_each(|v: Vec<u64>| {
            if let Some(collection) = map.get_mut(&v[0]) {
                collection.insert(v[1]);
            } else {
                map.insert(v[0], HashSet::from([v[0]]));
            }
        });

    map
}

fn create_digits(digits_to_analysis: HashMap<u64, HashSet<u64>>) -> u64 {
    let mut map = digits_to_analysis;
    let mut new_digits = vec![];

    while !map.is_empty() {
        let next_to_use = map.iter().find(|(_, prev)| prev.is_empty());

        if let Some((&key, _)) = next_to_use {
            new_digits.insert(0, key);
            map.remove(&key);

            for (_, prev) in map.iter_mut() {
                prev.remove(&key);
            }
        }
    }

    digits_to_num(&new_digits)
}

fn find_shortest_digit(filename: &str) -> u64 {
    create_digits(analysis(filename))
}

fn main() {
    println!("{:?}", find_shortest_digit("src/0079_keylog.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(find_shortest_digit("src/0079_keylog.txt"), 73162890)
    }
}
