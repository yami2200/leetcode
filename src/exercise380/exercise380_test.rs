pub use crate::exercise380::exercise380;

#[test]
fn test_randomized_set() {
    let mut randomized_set = exercise380::RandomizedSet::new();
    assert!(randomized_set.insert(1));
    assert!(!randomized_set.remove(2));
    assert!(randomized_set.insert(2));
    let values = vec![1,2];
    assert!(values.contains(&randomized_set.get_random()));
    assert!(randomized_set.remove(1));
    assert!(!randomized_set.insert(2));
    assert_eq!(randomized_set.get_random(), 2);
}