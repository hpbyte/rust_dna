use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut dict: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let key = target - num;
        let prev_res = dict.get(&key);

        match prev_res {
            Some(j) => return vec![*j, i as i32],
            None => {
                dict.insert(*num, i as i32);
            }
        }
    }

    vec![]
}
