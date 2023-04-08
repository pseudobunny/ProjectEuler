use itertools::Itertools;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Something went wrong reading the file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .collect()
}

fn line_to_bytes(line: &String) -> Vec<u8> {
    line.split(",").flat_map(|s| s.parse::<u8>()).collect()
}

fn possible_keys() -> Vec<Vec<u8>> {
    (0..128).permutations(3).unique().collect_vec()
}

fn decrypt_vec(encrypted_vec: &[u8], key: &[u8]) -> Vec<u8> {
    let repeated_key = key.iter().cycle();

    encrypted_vec
        .iter()
        .zip(repeated_key)
        .map(|(encrypted_byte, key_byte)| encrypted_byte ^ key_byte)
        .collect_vec()
}

const THE_ASCII: [u8; 5] = [32, 116, 104, 101, 32];
fn contains_words(decrypted_vec: &[u8]) -> bool {
    decrypted_vec
        .windows(THE_ASCII.len())
        .any(|word_window| word_window == THE_ASCII)
}

fn decrypt_file(filename: &str) -> Option<Vec<u8>> {
    let encrypted_bytes = lines_from_file(filename)
        .iter()
        .flat_map(|line| line_to_bytes(line))
        .collect::<Vec<u8>>();

    let keys = possible_keys();

    keys.iter()
        .map(|key| decrypt_vec(&encrypted_bytes, key))
        .find(|decrypted_vec| contains_words(decrypted_vec))
}

fn find_decrypted_file_sum(filename: &str) -> Option<u32> {
    let decrypted_file = decrypt_file(filename);

    match decrypted_file {
        Some(decrypted_vec) => Some(decrypted_vec.iter().map(|&byte| byte as u32).sum::<u32>()),
        None => None,
    }
}

fn main() {
    println!("{}", find_decrypted_file_sum("p059_cipher.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        let sum = find_decrypted_file_sum("p059_cipher.txt");
        assert_eq!(sum.unwrap(), 129448);
    }
}
