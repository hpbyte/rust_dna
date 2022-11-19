## Problem

https://leetcode.com/problems/find-pivot-index/

## Optimal Approach [O(n)]

- find the total_sum first
- initialize the left_sum
- loop through the nums array
  - calculate right_sum = total_sum - left_sum - num
  - if left_sum == right_sum, return the index
  - else, add num into left_sum
- loop ends, return -1 not found
