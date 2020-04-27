# Two Sum

## Description

Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

## Examples

> Given nums = [2, 7, 11, 15], target = 9,
>
> Because nums[0] + nums[1] = 2 + 7 = 9,
> return [0, 1].

## Solutions

|     | Language |
| --- | -------- |
|  ✅ | [Python](https://github.com/andrewleverette/leetcode_problems/blob/master/two_sum/python/solution.py)|
|  ✅ | [Rust](https://github.com/andrewleverette/leetcode_problems/blob/master/two_sum/rust/solution/src/lib.rs) |

### Approach 1: Brute Force

The naive approach to solving this problem is to loop through each element `x` in `nums` and then loop through each element again to find if there is a value that equals `target` - `x`

#### Algorithm

```
for i = 0 to length of nums
    for j = i + 1 to length of nums
        if nums[j] == target - nums[i]
            return [i, j]
```

#### Complexity

- Time complexity: O(n^2) since for each element we loop through of the array
- Space complexity: O(1)