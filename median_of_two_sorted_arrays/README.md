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
    if i == length of nums1
        merged_array[k] = nums1[j]
        j++
    else if j == length of nums2
        merged_array[k] = nums2[i]
        j++
    else
        if nums1[i] <= nums2[j]
            merged_array[k] = nums2[i]
            i++
        else
            merged_array[k] = nums1[j]
            j++
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

### Approach 2: Intuition

The above solution can be improved further. The median essentially divides a set of numbers in half. Since we are given two sets of numbers, instead of building a merged set, we can partition each set in halves such that all the elements in the first half are less than or equal to all the elements in the second half.

The partition index for `nums1` is initial just `n / 2` where `n` is the length of `nums1`.

The partition index for `nums2` is `(n + m + 1) / 2 - i` where `n` is the length of `nums1` and `m` is the length of `nums2` and `i` is the partition index of `nums1`.

Then you adjust the partition indices until each value is in the correct half.

#### Example

Input: 

`nums1` = [2, 3]
`nums2` = [1, 4, 5]

Output: 3

Explanation:

n = 2

m = 3

min = 0, max = 2

Iteration 1:

    i = 1, j = 2

    first half -> [1, 2, 4], second half -> [3, 5]: 4 and 3 are in the wrong halves

    min = i + 1 = 2

Iteration 2:

    i = (2 + 2) / 2 = 2

    j = (2 + 3 + 1) / 2 - 2 = 1

    first half -> [1, 2, 3], second half -> [4, 5]: all values are in the right place

    median = max of first half -> max(nums1[i - 1], nums2[j - 1])

The length of both array is odd so just return median.

> Note: n should be less than m otherwise you could get a negative value for j

#### Algorithm

```
n = length of nums1
m = length of nums2
min = 0
max = n
i, j = 0

while min <= max
    i = (min + max) / 2
    j = (n + m + 1) / 2 - i

    if i < n and j > 0 and nums2[j - 1] > nums[i]
        min = i + 1
    else if i > 0 j < m nums2[j] < nums1[i -1]
        max = i - 1
    else
        if i == 0
            median = nums2[j - 1]
        else if j == 0
            median = nums1[i - 1]
        else
            median = max(nums1[i - 1], nums2[j - 1])
```

#### Complexity

- Time complexity: O(log(min(n, m)))
- Space complexity: O(1) - No extra space is used.