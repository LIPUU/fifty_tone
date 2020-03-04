use rand::{Rng};
use std::collections::HashMap;

pub fn exchange(nums: &mut Vec<usize>, l: usize, r: usize) {
    let tmp = nums[l];
    nums[l] = nums[r];
    nums[r] = tmp;
}

pub fn generate_random_sequence(keys_tones: &HashMap<usize, &&str>) {
    let keys_tones_length = keys_tones.len();
    let mut nums: Vec<usize> = (0..keys_tones_length).collect();

    for i in 1..25 {
        for j in 0..keys_tones_length {
            let secret_number = rand::thread_rng().gen_range(0, keys_tones_length);
            exchange(&mut nums, j, secret_number);
        }
        for t in &nums {
            print!("{} ", keys_tones.get(t).unwrap());
        }
        if i%3==0 {
            println!();
        }
    }
}


#[test]
fn exchage_test() {
    let mut nums: Vec<usize> = vec![1, 2, 3];
    exchange(&mut nums, 0, 2);
    assert_eq!(
        vec![3, 2, 1],
        nums
    );
}