pub use crate::exercise1143::exercise1143;

#[test]
fn test_longest_common_subsequence() {
    assert_eq!(exercise1143::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
    assert_eq!(exercise1143::longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
    assert_eq!(exercise1143::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
}