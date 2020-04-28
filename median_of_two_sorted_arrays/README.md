# 4. Median of Two Sorted Arrays

There are two sorted arrays `nums1` and `nums2` of size m and n respectively.

Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).

You may assume `nums1` and `nums2` cannot be both empty.

## Examples

### Example 1

> nums1 = [1, 3]
> nums2 = [2]
> 
> The median is 2.0

### Example 2

> nums1 = [1, 2]
> nums2 = [3, 4]
> 
> The median is (2 + 3)/2 = 2.5

## Solution

|     | Language |
| --- | -------- |
|  - | [Python](https://github.com/andrewleverette/leetcode_problems/blob/master/median_of_two_sorted_arrays/python/solution.py)|
|  ✅ | [Rust](https://github.com/andrewleverette/leetcode_problems/blob/master/median_of_two_sorted_arrays/rust/solution/src/lib.rs) |

### Approach 1: Brute Force

Since both input arrays are already sorted, a naive solution would be to merge the arrays by iterating through each and inserting the items into a new array. Then calculate the median.

#### Algorithm

```
merged_array: length of nums1 + nums2
int i = 0
int j = 0
int k = 0
while i < length of nums1 or j < length of nums2
    if nums1[i] <= nums2[j]
        merged_array[k] = nums1[i]
        i++
    else
        merged_array[k] = nums2[j]
    k++
mid = length of merged array / 2
if length of merged_array is odd
    return merged_array[mid]
else

    return (merged_array[mid] + merged_array[mid - 1]) / 2
```

#### Complexity

- Time complexity: O(m + n) -> O(n) - Iterating over both arrays in the while loop means that this has a linear runtime
- Space complexity: O(m + n) -> O(n) - The extra array that is the result of merging the two input arrays has a linear space requirement