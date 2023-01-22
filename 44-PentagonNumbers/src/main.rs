use custom_math_utilities::penta_number_list;

fn min_penta_distance() -> u64 {
    let mut found = false;
    let mut min_d = 0;
    let mut penta_list = penta_number_list();

    let mut i = 1;
    let view_range = 1000;
    while !found {
        let a = penta_list.get(i);
        for j in 1..=view_range {
            let b = penta_list.get(i+j);
            
            if penta_list.is_in(a+b) {
                if penta_list.is_in(b+b+a) {
                    min_d = a;
                    found = true;
                    break;
                }
            }
            if penta_list.is_in(b-a) {
                if penta_list.is_in(b+b-a) {
                    min_d = a;
                    found = true;
                    break;
                }
            }
        }

        i += 1;
    }

    min_d
}

fn main () {
    println!("{}", min_penta_distance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(min_penta_distance(), 5482660);
    }
}
