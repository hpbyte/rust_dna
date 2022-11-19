pub fn is_subsequence(s: String, t: String) -> bool {
    let mut target = t.clone();
    for (_, c) in s.chars().enumerate() {
        let index = target.find(c);
        match index {
            Some(index) => {
                target = target[index + 1..].to_string();
            }
            None => return false,
        }
    }

    true
}
