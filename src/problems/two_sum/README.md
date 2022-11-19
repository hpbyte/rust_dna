## Problem

https://leetcode.com/problems/two-sum/

## Approach 1 [O(n^2)]

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}
```

## Optimal Approach [O(n)]

We can utilize a hash map to memorize the previous computed values.

Algorithm:

- create a hashmap
- loop through the nums array 
  - subtract the current num from the target sum
  - find the subtracted value in the hashmap
    - if exists, this means there was a value that can combine with the current num to make up the target sum
    - else, push the value into the hashmap


