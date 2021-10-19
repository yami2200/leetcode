use std::cmp;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut length_max = strs[0].to_string();
    if length_max.len() == 0 { return "".to_string(); }
    for i in 0..strs.len(){
        let mut current = "".to_string().to_owned();
        if strs[i].len() == 0 { return "".to_string() }
        for j in 0..cmp::min(strs[i].len(), length_max.len()){
            if strs[i].chars().nth(j).unwrap().eq(&length_max.chars().nth(j).unwrap()) { current.push(length_max.chars().nth(j).unwrap()); }
            else {break;}
        }
        if current.len() < length_max.len() { length_max = current.to_string(); }
    }
    length_max
}