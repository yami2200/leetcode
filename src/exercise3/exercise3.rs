use std::collections::HashMap;

// ################### QUITE SLOW
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut min_index : usize = 0;
    let mut map : HashMap<char, usize> = HashMap::new();
    for (index, char) in s.chars().enumerate(){
        if let Some(c) = map.get(&char) {
            for i in min_index..=*c{
                map.remove(&s.chars().nth(i).unwrap());
                min_index+=1;
            }
        }
        map.insert(char, index);
        if map.len() > max { max = map.len()}
    }
    max as i32
}