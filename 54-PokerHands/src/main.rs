use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use poker_hands::PokerHand;

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect("Something went wrong reading the file");
    
    let buf = BufReader::new(file);
    
    buf.lines()
       .map(|l| l.expect("Could not parse line").trim().to_string())
       .collect()
}

fn number_of_hands_p1_wins(filename: &str) -> usize {
    let poker_hands_both_players = lines_from_file(filename);

    let poker_hands_players_split = poker_hands_both_players.iter()
        .map(|phbp| phbp.split_whitespace().collect::<Vec<&str>>())
        .map(|phbpv| (phbpv[0..5].join(" "), phbpv[5..10].join(" ")))
        .map(|(p1, p2)| (PokerHand::new(&p1[..]), PokerHand::new(&p2[..])));

    poker_hands_players_split
        .filter(|(p1, p2)| p1 > p2)
        .count()
}

fn main() {
    println!("{}", number_of_hands_p1_wins("src/p054_poker.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(
            number_of_hands_p1_wins("src/p054_poker.txt"),
            376
        );
    }
}
