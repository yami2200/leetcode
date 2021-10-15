use std::collections::HashMap;

// ################### BRUTE FORCE
pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len(){
        for j in i+1..nums.len(){
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    Vec::new()
}

// ################## MORE OPTIMISED
pub fn two_sum2(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len(){
        if let Some(j) = map.get(&(target - nums[i])) { return vec![*j  as i32, i as i32]; }
        map.insert(nums[i], i);
    }
    Vec::new()
}