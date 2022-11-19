pub fn fibonacci_number(n: i32) -> i32 {
    let mut f = Vec::new();
    f.push(0);
    f.push(1);

    for i in 2..=n as usize {
        f.push(f[i - 1] + f[i - 2])
    }

    return f[n as usize];
}
