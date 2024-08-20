use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref FIRST_LETTER_VALUE_MAP: HashMap<char, i32> = {
        let mut map = HashMap::new();
        map.insert('A', 1);
        map.insert('B', 0);
        map.insert('C', 9);
        map.insert('D', 8);
        map.insert('E', 7);
        map.insert('F', 6);
        map.insert('G', 5);
        map.insert('H', 4);
        map.insert('I', 9);
        map.insert('J', 3);
        map.insert('K', 2);
        map.insert('L', 2);
        map.insert('M', 1);
        map.insert('N', 0);
        map.insert('O', 8);
        map.insert('P', 9);
        map.insert('Q', 8);
        map.insert('R', 7);
        map.insert('S', 6);
        map.insert('T', 5);
        map.insert('U', 4);
        map.insert('V', 3);
        map.insert('W', 1);
        map.insert('X', 3);
        map.insert('Y', 2);
        map.insert('Z', 0);
        map
    };
    static ref PATTERN: Regex = Regex::new(r"^[A-Z][12]\d{8}$").unwrap();
    static ref MULTIPLIERS: [i32; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
}

pub fn national_id_valid(identity_card_number: &str) -> bool {
    if identity_card_number.is_empty() {
        return false;
    }

    let upper_case_identity_card_number = identity_card_number.to_uppercase();

    if !valid_format(&upper_case_identity_card_number) {
        return false;
    }

    let identity_number_chars: Vec<char> = upper_case_identity_card_number.chars().collect();
    let check_sum = identity_number_chars[9].to_digit(10).unwrap() as i32;
    let calculate_check_sum = calculate_check_sum(&identity_number_chars);

    calculate_check_sum == check_sum
}

fn valid_format(taiwan_identity_number: &str) -> bool {
    PATTERN.is_match(taiwan_identity_number)
}

fn calculate_check_sum(identity_number_chars: &[char]) -> i32 {
    let first_letter = identity_number_chars[0];
    let mut sum = *FIRST_LETTER_VALUE_MAP.get(&first_letter).unwrap_or(&0);

    for i in 1..9 {
        let num = identity_number_chars[i].to_digit(10).unwrap() as i32;
        sum += num * MULTIPLIERS[i - 1];
    }

    let sum_mod_10 = sum % 10;
    if sum_mod_10 == 0 {
        0
    } else {
        10 - sum_mod_10
    }
}

#[cfg(test)]
mod tests {
    use crate::national_id_valid;

    #[test]
    fn test_national_id_valid() {
        assert_eq!(national_id_valid("A123456789"), true);
        assert_eq!(national_id_valid("X108165374"), true);
        assert_eq!(national_id_valid("G215254402"), true);
        assert_eq!(national_id_valid("W117266742"), true);

        assert_eq!(national_id_valid("A123456"), false);
        assert_eq!(national_id_valid("A12345678999999"), false);
        assert_eq!(national_id_valid("123456"), false);

        assert_eq!(national_id_valid("N116665474"), false);
        assert_eq!(national_id_valid("F106131710"), false);
        assert_eq!(national_id_valid("U114432640"), false);
        assert_eq!(national_id_valid("R213316215"), false);
        assert_eq!(national_id_valid("N116665473"), false);
        assert_eq!(national_id_valid("K252173159"), false);
    }
}
