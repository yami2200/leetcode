mod exercise1;
mod exercise9;

fn main() {
    test_exercise1();
    test_exercise9();
}

fn test_exercise1(){
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(exercise1::exercise1::two_sum(&nums, target), vec![0, 1]);
}

fn test_exercise9(){
    let x = 5665;
    assert!(exercise9::exercise9::is_palindrome(x));
}