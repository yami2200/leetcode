mod exercise1;

fn main() {
    test_exercise1();
}

fn test_exercise1(){
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", exercise1::exercise1::two_sum(&nums, target));
}