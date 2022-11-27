use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn check_primality(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n %3 == 0 {
        return false;
    }
    let mut d = 5;
    while d*d <= n {
        if n % d == 0 || n % (d + 2) == 0 {
            return false;
        }
        d += 6;
    }

    true
}

const MAX_PRIME_GENERATION: u64 = 10_000;
const SET_LENGTH: usize = 5;

fn is_prime_with_list(n: u64, primes: &mut Vec<u64>) -> bool {
    if n > MAX_PRIME_GENERATION {
        if check_primality(n) {
            primes.push(n);
            true
        } else {
            false
        }
    } else {
        primes.contains(&n)
    }
}

fn check_pair(pair: &[&u64], primes: &mut Vec<u64>) -> bool {
    let p1s = pair[0].to_string();
    let p2s = pair[1].to_string();

    let c1 = (p1s.clone() + &p2s).parse::<u64>().unwrap();
    let c2 = (p2s + &p1s).parse::<u64>().unwrap();

    is_prime_with_list(c1, primes) && is_prime_with_list(c2, primes)
}

fn get_other_num_in_pair(pair: &[u64], n: u64) -> u64 {
    if pair[0] == n {
        pair[1]
    } else {
        pair[0]
    }
}

fn get_other_num_in_all_pairs(pairs: Vec<&Vec<u64>>, prime: u64) -> Vec<u64> {
    let other_nums = pairs.iter().map(|pair| get_other_num_in_pair(pair, prime));
    let mut other_nums_set = HashSet::new();

    for n in other_nums {
        other_nums_set.insert(n);
    }

    other_nums_set.iter().copied().collect()
}

fn min_set_starter(n: u64, map: &HashMap<u64, Vec<u64>>) -> Option<Vec<u64>> {
    let possible_avenues = map.keys()
        .filter(|k| map[k].contains(&n));

    let results = possible_avenues.map(|a| min_set(vec![n,*a], map))
        .collect::<Vec<Vec<Vec<u64>>>>()
        .concat();
    
    let min_set = results.iter().filter(|v| v.len() == SET_LENGTH)
        .min_by(|a, b| a.iter().sum::<u64>().cmp(&b.iter().sum::<u64>()));

    min_set.cloned()
}

fn min_set(n_v: Vec<u64>, map: &HashMap<u64, Vec<u64>>) -> Vec<Vec<u64>> {
    let possible_avenues = map.keys()
        .filter(|k| n_v.iter().all(|n| map[k].contains(n)));

    if possible_avenues.clone().count() < 1 {
        return vec![n_v];
    }

    possible_avenues.map(|a| {
        let mut new_v = n_v.clone();
        new_v.push(*a);

        min_set(new_v, map)
    })
    .collect::<Vec<Vec<Vec<u64>>>>()
    .concat()
}

fn main() {
    let mut primes = (2..MAX_PRIME_GENERATION)
        .filter(|n| check_primality(*n))
        .collect::<Vec<u64>>();
    
    let prime_pairs = primes.clone()
        .iter()
        .permutations(2)
        .unique()
        .filter(|s| check_pair(&s[..], &mut primes))
        .map(|v| v.iter().map(|n| **n).collect())
        .collect::<Vec<Vec<u64>>>();

    let mut prime_to_pairs = HashMap::new();
    
    for prime in primes {
        let pairs = prime_pairs.iter()
            .filter(|pair| pair.contains(&prime))
            .collect::<Vec<&Vec<u64>>>();
        if !pairs.is_empty() {
            prime_to_pairs.insert(
                prime,
                get_other_num_in_all_pairs(pairs, prime)
            );
        }
    }

    let min_set = prime_to_pairs.keys()
        .filter_map(|k| min_set_starter(*k, &prime_to_pairs))
        .min_by(|a, b| a.iter().sum::<u64>().cmp(&b.iter().sum::<u64>()));

    println!("{}", min_set.unwrap().iter().sum::<u64>());
}
