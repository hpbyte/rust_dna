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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find() {
        assert_eq!(binary_search(vec![-1,0,3,5,9,12], 9), 4);
    }

    #[test]
    fn should_not_find_1() {
        assert_eq!(binary_search(vec![-1,0,3,5,9,12], 2), -1);
    }

    #[test]
    fn shoud_not_find_2() {
        assert_eq!(binary_search(vec![5], -5), -1);
    }
}
