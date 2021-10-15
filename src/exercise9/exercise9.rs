pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    return s.chars().rev().collect::<String>().eq(&s);
}