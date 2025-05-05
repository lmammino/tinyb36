// Rust Bytes Issue 65: Integer to Alphanumeric Conversion
const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn convert_to_base36(num: u32) -> String {
    if num == 0 {
        return ALPHABET[0].to_string();
    }

    let mut result = String::new();
    let mut n = num;
    while n > 0 {
        let remainder = (n % 36) as usize;
        result = format!("{}{}", ALPHABET[remainder], result);
        n /= 36;
    }

    result
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(convert_to_base36(0), "0");
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(convert_to_base36(35), "z");
    }

    #[test]
    fn test_two_digits() {
        assert_eq!(convert_to_base36(36), "10");
    }

    #[test]
    fn test_large_number() {
        assert_eq!(convert_to_base36(1295), "zz");
    }

    #[test]
    fn test_max_input() {
        assert_eq!(convert_to_base36(2147483647), "zik0zj");
    }

    #[test]
    fn test_boundary_minus_one() {
        assert_eq!(convert_to_base36(2147483646), "zik0zi");
    }

    #[test]
    fn test_small_number() {
        assert_eq!(convert_to_base36(1), "1");
    }

    #[test]
    fn test_mid_range() {
        assert_eq!(convert_to_base36(46656), "1000");
    }

    #[test]
    fn test_power_of_36() {
        assert_eq!(convert_to_base36(46656), "1000"); // 36^3
    }

    #[test]
    fn test_near_power_of_36() {
        assert_eq!(convert_to_base36(46655), "zzz"); // 36^3 - 1
    }
}
