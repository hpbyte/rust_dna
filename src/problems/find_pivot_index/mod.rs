pub fn pivot_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let total: i32 = nums.iter().sum();
    let mut left_sum = 0;

    for (index, num) in nums.iter().enumerate() {
        let right_sum = total - left_sum - num;

        if (left_sum == right_sum) {
            return index as i32;
        }

        left_sum += num;
    }

    -1
}
