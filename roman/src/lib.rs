pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn convert(roman_numeral: &str) -> u32 {
    let mut sum = 0;

    for ch in roman_numeral.chars() {
        println!("ch {}", ch);
        match ch {
            'I' => sum += 1,
            'V' => sum += 5,
            'X' => sum += 10,
            'L' => sum += 50,
            'C' => sum += 100,
            'D' => sum += 500,
            'M' => sum += 1000,
            _ => {}
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_should_pass() {
        assert_eq!(convert("d"), 1)
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_handle_additive_digits() {
        assert_eq!(3, convert("III"));
        assert_eq!(convert("MMMDCCCLXXXVIII"), 3888);
        assert_eq!(convert("VII"), 7)
    }
}
