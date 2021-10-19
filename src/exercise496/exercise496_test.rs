pub use crate::exercise496::exercise496;

#[test]
fn test_next_greater_element() {
    assert_eq!(exercise496::next_greater_element(vec![4,1,2], vec![1,3,4,2]), vec![-1,3,-1]);
    assert_eq!(exercise496::next_greater_element(vec![2,4], vec![1,2,3,4]), vec![3,-1]);
}