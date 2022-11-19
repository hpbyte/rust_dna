use std::cmp;

pub fn insertion_sort<T>(input: &mut Vec<T>)
where
    T: cmp::PartialOrd,
{
    if input.len() == 0 {
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::sort::is_sorted;

    #[test]
    fn should_sort_correctly() {
        let mut nums = vec![8, 4, 5, 9, 3, 0, 7, 2, 1, 6];
        insertion_sort(&mut nums);
        assert!(is_sorted(&nums));
    }

    #[test]
    fn should_not_need_to_sort_if_empty() {
        let mut nums = vec![];
        insertion_sort(&mut nums);
        assert!(is_sorted(&nums));
    }
}
