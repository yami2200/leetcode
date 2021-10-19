use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    if nums2.len() == 1 { return vec![-1] }
    let mut map : HashMap<&i32, usize> = HashMap::new();
    let mut result = vec![-1;nums1.len()];
    for i in 0..nums1.len() {
        map.insert(&nums1[i], i);
    }
    let mut stack = vec![nums2[0]];
    for i in 1..nums2.len() {
        while stack.last() != None && &nums2[i] > stack.last().unwrap(){
            if map.get(stack.last().unwrap()) != None {
                result[*map.get(stack.last().unwrap()).unwrap()] = nums2[i];
            }
            stack.pop();
        }
        stack.push(nums2[i]);
    }
    result
}