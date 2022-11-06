pub fn bubble_sort(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    let len = nums.len();

    for i in 0..len {
        let mut swapped = false;

        for j in 0..len - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sort::is_sorted;
    use super::*;

    #[test]
    fn should_sort_correctly() {
        let mut nums = vec![8,4,5,9,3,0,7,2,1,6];
        bubble_sort(&mut nums);
        assert!(is_sorted(&nums));
    }

    #[test]
    fn should_not_need_to_sort_if_empty() {
        let mut nums = vec![];
        bubble_sort(&mut nums);
        assert!(is_sorted(&nums));
    }
}
