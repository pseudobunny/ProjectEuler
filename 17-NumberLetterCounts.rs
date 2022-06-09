use std::env;

fn num_str_val_to_ind(n_str: String, ind: usize) -> usize {
    n_str.chars().nth(ind).unwrap().to_digit(10).unwrap() as usize
}

fn convert_to_letter_tens(n_str: String) -> u32 {
    let ones_place = vec![0,3,3,5,4,4,3,5,5,4];
    let tens_place_special = vec![3,6,6,8,8,7,7,9,8,8];
    let tens_place = vec![0,0,6,6,5,5,5,7,6,6];

    if n_str.chars().nth(0).unwrap() == '1' {
        return tens_place_special[num_str_val_to_ind(n_str, 1)];
    }
    else {
        return tens_place[num_str_val_to_ind(n_str.clone(), 0)]
               + ones_place[num_str_val_to_ind(n_str, 1)];
    }
}

fn convert_to_letters(n: u32) -> u32 {
    let digit_letters = vec![0,3,3,5,4,4,3,5,5,4];
    
    let n_str = n.to_string();

    if n_str.len() == 1 {
        return digit_letters[num_str_val_to_ind(n_str, 0)];
    }

    if n_str.len() == 2 {
        return convert_to_letter_tens(n_str);
    }

    if n_str.len() == 3 {
        let part_hundreds = digit_letters[num_str_val_to_ind(n_str.clone(), 0)];
        let part_tens_ones = convert_to_letter_tens(n_str[1..3].to_string());
        
        if part_tens_ones == 0 {
            return part_hundreds + 7;
        } else {
            return part_hundreds + 10 + part_tens_ones;
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let end_num = (&args[1]).parse::<u32>().expect("Improper end number in arguments");

    let mut sum: u32 = 0;
    for i in 1..end_num+1 {
        sum += convert_to_letters(i);
    }

    //println!("{}",convert_to_letters(end_num))
    println!("{}", sum)
}