pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = nums.len() as i32 - 1;

    let res = loop {
        if start > end {
            break -1;
        }

        let mid: i32 = (start + end) >> 1;

        if target == nums[mid as usize] {
            break mid;
        } else if target < nums[mid as usize] {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    };

    res
}
