pub use crate::exercise4::exercise4;

#[test]
fn test_find_median_sorted_arrays() {
    let error_margin = f64::EPSILON;
    assert!((exercise4::find_median_sorted_arrays(vec![1,3], vec![2]) - 2.0).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![1,2], vec![3,4]) - 2.5).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![0,0], vec![0,0])).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![], vec![3]) - 3.0).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![2], vec![]) - 2.0).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![1,2,3], vec![4,5,6,7]) - 4.0).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![1,2,3,7,9,15], vec![4,5,6,8,10,11,12,13,14]) - 8.0).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![], vec![2,3]) - 2.5).abs() <= error_margin);
    //println!("{}", exercise4::find_median_sorted_arrays(vec![1,5,6], vec![2,3,4,7,8]));
    assert!((exercise4::find_median_sorted_arrays(vec![1,5,6], vec![2,3,4,7,8]) - 4.5).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![2,2,4,4], vec![2,2,4,4]) - 3.0).abs() <= error_margin);
    assert!((exercise4::find_median_sorted_arrays(vec![1,4,5,6], vec![2,3,7,8]) - 4.5).abs() <= error_margin);
}