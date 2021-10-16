pub use crate::exercise13::exercise13;

#[test]
fn test_roman_to_int() {
    assert_eq!(exercise13::roman_to_int("III".parse().unwrap()), 3);
    assert_eq!(exercise13::roman_to_int("IV".parse().unwrap()), 4);
    assert_eq!(exercise13::roman_to_int("IX".parse().unwrap()), 9);
    assert_eq!(exercise13::roman_to_int("LVIII".parse().unwrap()), 58);
    assert_eq!(exercise13::roman_to_int("MCMXCIV".parse().unwrap()), 1994);
}