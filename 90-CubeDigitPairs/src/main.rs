use itertools::Itertools;
use std::collections::HashSet;
use std::iter::once;

fn extend_set(set: &[u64]) -> Vec<u64> {
    let has_6 = set.contains(&6);
    let has_9 = set.contains(&9);

    let mut copied_set: Vec<u64> = set.to_vec();
    if has_6 ^ has_9 {
        copied_set.push(if has_6 { 9 } else { 6 });
    }

    copied_set
}

const SQUARES: [u64; 9] = [1, 4, 9, 16, 25, 36, 49, 64, 81];
const SQUARE_DIGITS: [(u64, u64); 9] = [
    (0, 1),
    (0, 4),
    (0, 9),
    (1, 6),
    (2, 5),
    (3, 6),
    (4, 9),
    (6, 4),
    (8, 1),
];

enum OtherDigit {
    Required(u64),
    Optional(u64, u64),
    Unviable,
}

fn get_other_digits(n_v: &[u64]) -> Vec<OtherDigit> {
    SQUARE_DIGITS
        .iter()
        .map(|dt| {
            let c0 = n_v.contains(&dt.0);
            let c1 = n_v.contains(&dt.1);

            if c0 && c1 {
                OtherDigit::Optional(dt.0, dt.1)
            } else if c0 {
                OtherDigit::Required(dt.1)
            } else if c1 {
                OtherDigit::Required(dt.0)
            } else {
                OtherDigit::Unviable
            }
        })
        .collect()
}

fn check_set_viable(n_v: &[u64]) -> bool {
    let odv = get_other_digits(&extend_set(n_v));

    if odv.iter().any(|od| matches!(od, OtherDigit::Unviable)) {
        return false;
    }

    let mut required = vec![];
    let mut optional = vec![];
    for od in odv {
        match od {
            OtherDigit::Required(v) => required.push(v),
            OtherDigit::Optional(v1, v2) => optional.push((v1, v2)),
            _ => (),
        };
    }
    // Filter down required
    required = HashSet::<u64>::from_iter(required.iter().cloned())
        .iter()
        .copied()
        .collect();

    // Filter down optional
    optional.dedup_by(|a, b| (a.0 == b.0 && a.1 == b.1) || (a.1 == b.0 && a.0 == b.1));
    let mut temp_optional = vec![];
    for pair in optional.iter() {
        // remove clear cases (aka one of the options is required)
        let in_required = required.contains(&pair.0) || required.contains(&pair.1);
        if !in_required {
            temp_optional.push(*pair);
        }
    }
    optional = temp_optional;

    // Make optimal optional choices
    let all_options =
        HashSet::<u64>::from_iter(optional.iter().flat_map(|t| once(t.0).chain(once(t.1))))
            .iter()
            .copied()
            .collect::<Vec<u64>>();

    let best_options = (1..=all_options.len())
        .flat_map(|n| all_options.iter().permutations(n).unique())
        .map(|v| v.iter().map(|n| **n).collect::<Vec<u64>>())
        .map(|v| extend_set(&v))
        .find(|v| {
            optional
                .iter()
                .all(|p| v.contains(&p.0) || v.contains(&p.1))
        });

    match best_options {
        Some(v) => required.append(&mut v.to_vec()),
        None => return false,
    }

    let len = required.len()
        - if required.contains(&6) && required.contains(&9) {
            1
        } else {
            0
        };

    len <= 6
}

fn check_if_sets_match(a: &[u64], b: &[u64]) -> bool {
    let a_e = extend_set(a);
    let b_e = extend_set(b);

    let created_digits = a_e
        .iter()
        .flat_map(|a_n| {
            b_e.iter()
                .map(|b_n| (*a_n, *b_n))
                .collect::<Vec<(u64, u64)>>()
        })
        .flat_map(|(a, b)| vec![a * 10 + b, b * 10 + a])
        .collect::<Vec<u64>>();

    SQUARES.iter().all(|d| created_digits.contains(d))
}

fn count_of_viable_pairs() -> usize {
    (0..10)
        .combinations(6)
        .unique()
        .filter(|s| check_set_viable(s))
        .combinations(2)
        .unique()
        .filter(|v| check_if_sets_match(&v[0], &v[1]))
        .count()
}

fn main() {
    println!("{}", count_of_viable_pairs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(count_of_viable_pairs(), 1217);
    }
}
