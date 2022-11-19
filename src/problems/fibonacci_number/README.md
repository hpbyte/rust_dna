## Problem

https://leetcode.com/problems/fibonacci-number/

## Approach 1 (Recursive)

```rust
pub fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    
    return Self::fib(n - 1) + Self::fib(n - 2);
}
```

## Optimal Approach [O(n)]

- make a new vector
- add the first two items 0 and 1
- loop from the third element to the end
  - add the result of combining previous two elements
- return the `n` index from the vector

