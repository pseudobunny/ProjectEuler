use std::collections::HashMap;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum MonopolySquare {
    Go = 0,
    A1 = 1,
    CC1 = 2,
    A2 = 3,
    T1 = 4,
    R1 = 5,
    B1 = 6,
    CH1 = 7,
    B2 = 8,
    B3 = 9,
    Jail = 10,
    C1 = 11,
    U1 = 12,
    C2 = 13,
    C3 = 14,
    R2 = 15,
    D1 = 16,
    CC2 = 17,
    D2 = 18,
    D3 = 19,
    FP = 20,
    E1 = 21,
    CH2 = 22,
    E2 = 23,
    E3 = 24,
    R3 = 25,
    F1 = 26,
    F2 = 27,
    U2 = 28,
    F3 = 29,
    G2J = 30,
    G1 = 31,
    G2 = 32,
    CC3 = 33,
    G3 = 34,
    R4 = 35,
    CH3 = 36,
    H1 = 37,
    T2 = 38,
    H2 = 39,
}

const COMMUNITY_CHEST_SQUARES: [MonopolySquare; 3] = [
    MonopolySquare::CC1,
    MonopolySquare::CC2,
    MonopolySquare::CC3,
];

const CHANCE_SQUARES: [MonopolySquare; 3] = [
    MonopolySquare::CH1,
    MonopolySquare::CH2,
    MonopolySquare::CH3,
];

const RAILROAD_SQUARES: [MonopolySquare; 4] = [
    MonopolySquare::R1,
    MonopolySquare::R2,
    MonopolySquare::R3,
    MonopolySquare::R4,
];

const UTILITY_SQUARES: [MonopolySquare; 2] = [MonopolySquare::U1, MonopolySquare::U2];

const MONOPOLY_SQUARE_IND_MAP: [MonopolySquare; 40] = [
    MonopolySquare::Go,
    MonopolySquare::A1,
    MonopolySquare::CC1,
    MonopolySquare::A2,
    MonopolySquare::T1,
    MonopolySquare::R1,
    MonopolySquare::B1,
    MonopolySquare::CH1,
    MonopolySquare::B2,
    MonopolySquare::B3,
    MonopolySquare::Jail,
    MonopolySquare::C1,
    MonopolySquare::U1,
    MonopolySquare::C2,
    MonopolySquare::C3,
    MonopolySquare::R2,
    MonopolySquare::D1,
    MonopolySquare::CC2,
    MonopolySquare::D2,
    MonopolySquare::D3,
    MonopolySquare::FP,
    MonopolySquare::E1,
    MonopolySquare::CH2,
    MonopolySquare::E2,
    MonopolySquare::E3,
    MonopolySquare::R3,
    MonopolySquare::F1,
    MonopolySquare::F2,
    MonopolySquare::U2,
    MonopolySquare::F3,
    MonopolySquare::G2J,
    MonopolySquare::G1,
    MonopolySquare::G2,
    MonopolySquare::CC3,
    MonopolySquare::G3,
    MonopolySquare::R4,
    MonopolySquare::CH3,
    MonopolySquare::H1,
    MonopolySquare::T2,
    MonopolySquare::H2,
];

impl MonopolySquare {
    fn to_modal_string(self) -> String {
        let original_string = (self as i64).to_string();

        if original_string.len() < 2 {
            format!("0{original_string}")
        } else {
            original_string
        }
    }

    fn from_ind(ind: usize) -> MonopolySquare {
        MONOPOLY_SQUARE_IND_MAP[ind % 40]
    }

    fn next_r_from<'a>(position: MonopolySquare) -> &'a MonopolySquare {
        Self::next_square_from_sequence(position, &RAILROAD_SQUARES)
    }

    fn next_u_from<'a>(position: MonopolySquare) -> &'a MonopolySquare {
        Self::next_square_from_sequence(position, &UTILITY_SQUARES)
    }

    fn next_square_from_sequence(
        position: MonopolySquare,
        sequence: &[MonopolySquare],
    ) -> &MonopolySquare {
        let position_ind = MONOPOLY_SQUARE_IND_MAP
            .iter()
            .position(|square| square == &position)
            .unwrap_or_default();

        MONOPOLY_SQUARE_IND_MAP
            .iter()
            .cycle()
            .skip(position_ind + 1)
            .find(|square| sequence.contains(square))
            .unwrap() // we are guaranteeed to find one
    }
}

#[derive(Clone, Copy)]
enum CommunityChestCard {
    AdvanceToGo,
    GoToJail,
    Nothing,
}

const COMMUNITY_CHEST: [CommunityChestCard; 16] = [
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::Nothing,
    CommunityChestCard::AdvanceToGo,
    CommunityChestCard::GoToJail,
];

#[derive(Clone, Copy)]
enum ChanceCard {
    AdvanceToGo,
    GoToJail,
    GoToC1,
    GoToE3,
    GoToH2,
    GoToR1,
    GoToNextR,
    GoToNextU,
    GoBack3Squares,
    Nothing,
}

const CHANCE: [ChanceCard; 16] = [
    ChanceCard::Nothing,
    ChanceCard::Nothing,
    ChanceCard::Nothing,
    ChanceCard::Nothing,
    ChanceCard::Nothing,
    ChanceCard::Nothing,
    ChanceCard::AdvanceToGo,
    ChanceCard::GoToJail,
    ChanceCard::GoToC1,
    ChanceCard::GoToE3,
    ChanceCard::GoToH2,
    ChanceCard::GoToR1,
    ChanceCard::GoToNextR,
    ChanceCard::GoToNextR,
    ChanceCard::GoToNextU,
    ChanceCard::GoBack3Squares,
];

struct Deck<T> {
    deck: Vec<T>,
}

impl<T: Clone + Copy> Deck<T> {
    fn new(cards: &[T]) -> Deck<T> {
        let mut deck = cards.to_vec();

        deck.shuffle(&mut rand::thread_rng());

        Deck::<T> { deck }
    }

    fn draw(&mut self) -> T {
        let card = self.deck.pop().unwrap(); // constructor ensures it's a full deck
        self.deck.insert(0, card);
        card
    }
}

struct Die {
    rng: ThreadRng,
    dist: Uniform<usize>,
}

impl Die {
    fn new(faces: usize) -> Die {
        Die {
            rng: rand::thread_rng(),
            dist: Uniform::from(1..=faces),
        }
    }

    fn roll(&mut self) -> usize {
        self.dist.sample(&mut self.rng)
    }
}

struct MonopolyGame<'a> {
    position: MonopolySquare,
    doubles_streak: u64,
    die: &'a mut Die,
    community_chest: Deck<CommunityChestCard>,
    chance: Deck<ChanceCard>,
    stats: HashMap<MonopolySquare, u64>,
}

impl MonopolyGame<'_> {
    fn new(die: &mut Die) -> MonopolyGame<'_> {
        MonopolyGame {
            position: MonopolySquare::Go,
            doubles_streak: 0,
            die,
            community_chest: Deck::new(&COMMUNITY_CHEST),
            chance: Deck::new(&CHANCE),
            stats: HashMap::new(),
        }
    }

    fn play_turn(&mut self) {
        let dice_roll_1 = self.die.roll();
        let dice_roll_2 = self.die.roll();

        if dice_roll_1 == dice_roll_2 {
            self.doubles_streak += 1;
        } else {
            self.doubles_streak = 0;
        }

        if self.doubles_streak > 3 {
            self.doubles_streak = 0;
            self.position = MonopolySquare::Jail;
            self.update_stats();
            return;
        }

        let move_position_ind = (self.position as usize) + dice_roll_1 + dice_roll_2;
        let move_position = MonopolySquare::from_ind(move_position_ind);

        if move_position == MonopolySquare::G2J {
            self.position = MonopolySquare::Jail;
        } else if COMMUNITY_CHEST_SQUARES.contains(&move_position) {
            let card = self.community_chest.draw();

            self.position = match card {
                CommunityChestCard::AdvanceToGo => MonopolySquare::Go,
                CommunityChestCard::GoToJail => MonopolySquare::Jail,
                CommunityChestCard::Nothing => move_position,
            }
        } else if CHANCE_SQUARES.contains(&move_position) {
            let card = self.chance.draw();

            self.position = match card {
                ChanceCard::AdvanceToGo => MonopolySquare::Go,
                ChanceCard::GoToJail => MonopolySquare::Jail,
                ChanceCard::GoToC1 => MonopolySquare::C1,
                ChanceCard::GoToE3 => MonopolySquare::E3,
                ChanceCard::GoToH2 => MonopolySquare::H2,
                ChanceCard::GoToR1 => MonopolySquare::R1,
                ChanceCard::GoToNextR => *MonopolySquare::next_r_from(move_position),
                ChanceCard::GoToNextU => *MonopolySquare::next_u_from(move_position),
                ChanceCard::GoBack3Squares => MonopolySquare::from_ind(move_position_ind - 3), // safe, as all chance squares are > 3
                ChanceCard::Nothing => move_position,
            }
        } else {
            self.position = move_position;
        }

        self.update_stats();
    }

    fn update_stats(&mut self) {
        let prev = self.stats.get(&self.position).unwrap_or(&0);
        self.stats.insert(self.position, prev + 1);
    }

    fn get_modal_string(self) -> String {
        let mut stats_vec = self.stats.iter().collect::<Vec<(&MonopolySquare, &u64)>>();
        stats_vec.sort_by(|(_, a), (_, b)| a.cmp(b));

        stats_vec
            .iter()
            .rev() // sorts by ascending, we want the top 3
            .take(3)
            .map(|(square, _)| square.to_modal_string())
            .collect()
    }
}

fn get_modal_string_for_die_faces(die_faces: usize) -> String {
    let mut die = Die::new(die_faces);
    let mut game = MonopolyGame::new(&mut die);

    for _ in 0..10_000_000 {
        game.play_turn();
    }

    game.get_modal_string()
}

fn main() {
    println!("{}", get_modal_string_for_die_faces(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        assert_eq!(get_modal_string_for_die_faces(6), "102400".to_string());
    }

    #[test]
    fn q_case() {
        assert_eq!(get_modal_string_for_die_faces(4), "101524".to_string());
    }
}
