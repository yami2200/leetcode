mod exercise1;
mod exercise9;
mod exercise13;

fn main() {
    test_exercise1();
    test_exercise9();
    test_exercise13();
}

fn test_exercise1(){
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(exercise1::exercise1::two_sum(&nums, target), vec![0, 1]);
    assert_eq!(exercise1::exercise1::two_sum2(&nums, target), vec![0, 1]);
}

fn test_exercise9(){
    let x = 5665;
    assert!(exercise9::exercise9::is_palindrome(x));
}

fn test_exercise13(){
    let mut s : String = "III".parse().unwrap();
    assert_eq!(exercise13::exercise13::roman_to_int(s), 3);
    s = "IV".parse().unwrap();
    assert_eq!(exercise13::exercise13::roman_to_int(s), 4);
    s = "IX".parse().unwrap();
    assert_eq!(exercise13::exercise13::roman_to_int(s), 9);
    s = "LVIII".parse().unwrap();
    assert_eq!(exercise13::exercise13::roman_to_int(s), 58);
    s = "MCMXCIV".parse().unwrap();
    assert_eq!(exercise13::exercise13::roman_to_int(s), 1994);
}