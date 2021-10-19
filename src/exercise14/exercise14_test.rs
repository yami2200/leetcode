pub use crate::exercise14::exercise14;

#[test]
fn test_longest_common_prefix() {
    assert_eq!(exercise14::longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl");
    assert_eq!(exercise14::longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]), "");
    assert_eq!(exercise14::longest_common_prefix(vec!["carton".to_string(),"carré".to_string(),"car".to_string()]), "car");
}