use std::fs::read_to_string;

fn main() {
    let mut all_calibration_values: u32 = 0;
    let lines: Vec<String> = read_to_string("input.txt")
                    .unwrap()
                    .lines()
                    .map(String::from)
                    .collect();

    for line in lines.iter() {
        let mut found_first_digit = false;
        let mut first_digit_char: char = '0';
        let mut last_digit_char: char = '0';
        for c in line.chars() {
            if is_character_digit(c) {
                if !found_first_digit {
                    first_digit_char = c;
                    last_digit_char = c;
                    found_first_digit = true;
                } else {
                    last_digit_char = c;
                }
            }
        }

        let mut number_str: String = String::from(first_digit_char);
        number_str.push(last_digit_char);
        let number: u32 = number_str.parse().unwrap();
        all_calibration_values += number;
    }

    println!("all calibration values sum to: {}", all_calibration_values)
}

fn is_character_digit(c: char) -> bool {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => return true,
        _ => return false
    }
}