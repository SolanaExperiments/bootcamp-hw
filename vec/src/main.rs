use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new(); // value -> index

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }

    vec![] // return empty if no solution (optional depending on constraints)
}

fn main() {
    println!("{:?}", two_sum(vec![2, 3, 4, 5], 9)); // should print [1, 3]
}
