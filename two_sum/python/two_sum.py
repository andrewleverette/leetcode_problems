from typing import List

# Returns the indices of two numbers that add up to a specific target
# 
# Arguments
# 
# * `nums` - A vector of integers
# * `target` - Result of the sum of two numbers in `nums`
# 
# Approach
# 
# This solution uses a HashMap to store the complement - `target` - `num`.
# If a complement is seen in `nums` then return a vector containing the current index
# and the complements index.

def two_sum(nums: List[int], target: int) -> List[int]:
    complements = {}
    for i, num in enumerate(nums):
        if num in complements:
            return [complements[num], i]
        else:
            complements[target-num] = i


def test_two_sum():
    nums = [2, 7, 11, 15]
    target = 9
    assert two_sum(nums, target) == [0, 1]