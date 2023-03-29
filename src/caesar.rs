use std::collections::HashMap;

const ASCII_A: i8 = 'a' as i8;
const ASCII_Z: i8 = 'z' as i8;
const ABC_SIZE: i8 = ASCII_Z - ASCII_A + 1;

pub fn run() {
    let text = "d fdw kdg orwv ri ixq sodblqj zlwk d judb edoo";
    let map = count_chars(&text);

    for (k, v) in map.iter() {
        println!("{k}:  {v}");
    }

    let mut shifted = String::from("");

    for c in text.chars() {
        shifted.push(char_shift(c, -3));
    }

    println!("{}", shifted);
}

pub fn count_chars(text: &str) -> HashMap<char, i32>{
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    for c in text.chars() {
        if c != ' ' {
            let entry = char_counts.entry(c).or_insert(0);
            *entry += 1;
        }
    }

    return char_counts;
}

pub fn char_shift(input: char, shift: i8) -> char{
    if !input.is_ascii_alphabetic() {
        return input;
    }

    if shift >= ABC_SIZE || shift <= -ABC_SIZE {
        panic!("Invalid shift!");
    }

    let input_lower = input.to_ascii_lowercase();

    let input_index = input_lower as i8 - ASCII_A;

    let output_index = (input_index + shift).rem_euclid(ABC_SIZE);

    let output = ((output_index + ASCII_A) as u8) as char;

    return output;
}