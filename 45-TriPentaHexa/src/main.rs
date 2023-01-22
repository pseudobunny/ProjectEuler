use custom_math_utilities::triangle_number_list;
use custom_math_utilities::penta_number_list;
use custom_math_utilities::hexa_number_list;

fn first_tri_penta_hexa_after_ind(n: usize) -> u64 {
    let mut tri_list  = triangle_number_list();
    let mut penta_list = penta_number_list();
    let mut hexa_list = hexa_number_list();

    let mut i = n;
    loop {
        let a = tri_list.get(i);

        if penta_list.is_in(a) && hexa_list.is_in(a) {
            return a;
        }

        i += 1;
    }
}

fn main () {
    println!("{}", first_tri_penta_hexa_after_ind(286))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_case() {
        assert_eq!(first_tri_penta_hexa_after_ind(286), 1533776805);
    }
}