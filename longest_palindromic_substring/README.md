# 5. Longest Palindromic Substring

Given a string `s`, find the longest palindromic substring in `s`. You may assume that the maximum length of `s` is 1000.

## Examples

### Example 1

> Input: "babad"
>
> Output: "bab"
>
> Note: "aba" is also a valid answer.

### Example 2

> Input: "cbbd"
>
> Output: "bb"

## Solution

|     | Language |
| --- | -------- |
|  - | Python |
|  ✅ | [Rust](https://github.com/andrewleverette/leetcode_problems/blob/master/longest_palindromic_substring/rust/solution/src/lib.rs) |

### Approach 1: Expand Around Center

A palindrome is a mirror around its center. That means that to test if a substring is a palindrome, we can just expand around the center. There are 2n - 1 centers since the center of a palindrome that has an even length is between the middle two characters.

#### Algorithm

```
n = length of s
start = 0
end = 0

for i = 0 to n
    left = i
    right = i

    while right + 1 and s[right + 1] == s[left]
        right += 1

    while left > 0 and right < n - 1 and s[left - 1] == s[right + 1]
        left -= 1
        right += 1

    if right - left > end - start
        start = left
        end = right

s[start..=end]
```

#### Complexity Analysis

- Time complexity: O(n^2)
- Space complexity: O(1)