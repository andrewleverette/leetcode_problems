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

### Approach 2: Two-pass Hash Table

The time complexity can be reduced, by converting the input array into a hash map that stores the value `nums[i]` as the key and the index `i` as the value. This would involve two iterations, one to build the hash map and another to check if the complement - `target - nums[i]` - exists in the hash map but is not the same index.

#### Algorithm

```
map = new HashMap
for i = 0 to length of nums
    map[nums[i]] = i

for i = 0 to length of nums
    complement = target - nums[i]
    if map.contains(complement) and map[complement] != i
        return [i, map[complement]]
```

#### Complexity

- Time complexity: O(2n) -> O(n) - The `nums` array is traversed twice, once to build the hash map and again to check if the complement has been seen.
- Space complexity: O(n) - The extra space for the hash map.

### Approach 3: One-pass Hash Table

The above solution can be improved even further by only checking if a compliment for a value has already been seen. If the complement hasn't been seen, add the complement and the index to the hash map, otherwise a solution has been found.

#### Algorithm

```
map = new HashMap
for i in length of nums
    if map.contains(num)
        return [map[num], i]
    else
        map.insert(target - num, i)
```