use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let list_symbol : HashMap<char,i32> = [('M', 1000i32), ('D', 500i32), ('C', 100i32), ('L', 50i32), ('X', 10i32), ('V', 5i32), ('I', 1i32), (' ', 0i32)].iter().cloned().collect();
    let mut total = 0;
    for i in 0..s.chars().count(){
        let c = list_symbol.get(&s.chars().nth(i).unwrap()).unwrap();
        let next = list_symbol.get(&s.chars().nth(i+1).unwrap_or(' ')).unwrap();
        if !next.eq(&0) && c<next{
            total-=c;
        } else {
            total+=c;
        }
    }
    total
}