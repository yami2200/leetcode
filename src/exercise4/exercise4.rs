// ###################### A SHAME :
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut vec = nums1.clone();
    let mut vec2 = nums2.clone();
    vec.append(&mut vec2);
    vec.sort();
    find_median_array(vec.to_vec())
}

pub fn find_median_array(nums: Vec<i32>) -> f64 {
    if nums.is_empty() { -1.0 }
    else if nums.len()%2 == 0 { (nums[nums.len() / 2] + nums[nums.len() / 2 - 1]) as f64 / 2.0}
    else { nums[nums.len() / 2] as f64 }
}

// not working :(
/*pub fn find_median_sorted_arrays2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {find_median_sorted_arrays_in_order(nums2, nums1)}
    else {find_median_sorted_arrays_in_order(nums1, nums2)}
}

pub fn find_median_sorted_arrays_in_order(smaller_array: Vec<i32>, bigger_array: Vec<i32>) -> f64{
    let sl = smaller_array.len();
    let bl = bigger_array.len();
    println!("{:?}", smaller_array);
    println!("{:?}", bigger_array);
    if smaller_array.is_empty() { return find_median_array(bigger_array)}
    if sl == 1 {
        if bl == 1 { return find_median_array(vec![smaller_array[0], bigger_array[0]])}
        if bl % 2 == 1 { return find_median_2i(bigger_array[bl/2], find_median_3i(smaller_array[0], bigger_array[bl/2 + 1], bigger_array[bl/2 - 1]) as i32)}
        else { return find_median_3i(bigger_array[bl/2], smaller_array[0], bigger_array[bl/2 - 1])}
    }
    if sl == 2 {
        if bl == 2 { return find_median_4i(smaller_array[0], smaller_array[1], bigger_array[0], bigger_array[1]) }
        if bl % 2 == 1 { return find_median_3i(bigger_array[bl / 2], cmp::max(smaller_array[0], bigger_array[bl/2 - 1]), cmp::min(smaller_array[1], bigger_array[bl/2 + 1])) }
        else { return find_median_4i(bigger_array[bl/2], bigger_array[bl/2 - 1], cmp::max( smaller_array[0], bigger_array[bl/2 - 2] ), cmp::min( smaller_array[1], bigger_array[bl/2 + 1] ))}
    }
    let i_s = (sl - 1) / 2;
    let i_b = (bl - 1) / 2;
    if smaller_array[i_s] <= bigger_array[i_b] { return find_median_sorted_arrays_in_order(smaller_array[i_s..sl].to_vec(), bigger_array[0..bl-i_s].to_vec()); }
    find_median_sorted_arrays_in_order(smaller_array[0..=i_s].to_vec(), bigger_array[i_s..bl].to_vec())
}



pub fn find_median_2i(n1: i32, n2: i32) -> f64{
    (n1 + n2) as f64 / 2.0
}

pub fn find_median_3i(n1: i32, n2: i32, n3: i32) -> f64{
    ((n1 + n2 + n3) - cmp::max(n1, cmp::max(n2,n3)) - cmp::min(n1, cmp::min(n2,n3))) as f64
}

pub fn find_median_4i(n1: i32, n2: i32, n3: i32, n4: i32) -> f64{
    ((n1 + n2 + n3 + n4) - cmp::max(n1, cmp::max(n2,cmp::max(n4,n3))) - cmp::min(n1, cmp::min(n2,cmp::min(n4,n3)))) as f64 / 2.0
}
*/