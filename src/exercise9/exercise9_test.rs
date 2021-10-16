pub use crate::exercise9::exercise9;

#[test]
fn test_is_palindrome() {
    assert!(exercise9::is_palindrome(5665));
    assert!(exercise9::is_palindrome(555));
    assert!(!exercise9::is_palindrome(56635));
}