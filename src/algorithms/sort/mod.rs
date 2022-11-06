pub mod bubble_sort;

pub fn is_sorted(input: &Vec<i32>) -> bool {
    if input.is_empty() {
        return true
    }

    let mut prev = &input[0];
    for i in input.iter().skip(1) {
        if prev > i {
            return false
        }
        prev = i
    }

    true
}
