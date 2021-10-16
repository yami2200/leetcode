pub use crate::exercise3::exercise3;

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(exercise3::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(exercise3::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(exercise3::length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(exercise3::length_of_longest_substring("".to_string()), 0);
    assert_eq!(exercise3::length_of_longest_substring("dvdf".to_string()), 3);
}