pub use crate::exercise1::exercise1;

#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(exercise1::two_sum(&nums, target), vec![0, 1]);
    assert_eq!(exercise1::two_sum2(&nums, target), vec![0, 1]);
}