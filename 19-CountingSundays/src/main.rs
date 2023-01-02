fn feb_days_by_year(n: u64) -> u64 {
    if n % 4 == 0 && (!(n % 100 == 0) || n % 400 == 0) {
        return 29;
    }

    28
}

fn month_days_by_year(month: u64, year: u64) -> u64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => feb_days_by_year(year),
    }
}

fn sundays_first_month_twentieth_century() -> u64 {
    let mut sum = 0;
    let mut day = 1;
    for year in 1900..=2000 {
        for month in 1..=12 {
            if day % 7 == 0 && year != 1900 {
                sum += 1;
            }
            day += month_days_by_year(month, year);
        }
    }

    sum
}

fn main() {
    println!("{}", sundays_first_month_twentieth_century())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(sundays_first_month_twentieth_century(), 171)
    }
}

