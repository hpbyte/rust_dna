## Problem

https://leetcode.com/problems/two-sum/

## Approach 1 [O(n^2)]



## Optimal Approach [O(n)]

We can utilize a hash map to memorize the previous computed values.

Algorithm:

- create a hashmap
- loop through the nums array 
  - subtract the current num from the target sum
  - find the subtracted value in the hashmap
    - if exists, this means there was a value that can combine with the current num to make up the target sum
    - else, push the value into the hashmap


