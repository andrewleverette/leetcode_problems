from typing import List


def two_sum_brute_force(nums: List[int], target: int) -> List[int]:
    """
    Returns the indices of two numbers that add up to a specific target

    Arguments

    * `nums` - A vector of integers
    * `target` - Result of the sum of two numbers in `nums`

    Approach

    This naive solution loops through each element `x` in `nums` and 
    then loops through each element again to find if there is a value 
    that equals `target` - `x`.
    """
    for i in range(len(nums)):
        for j in range(i + 1, len(nums)):
            if nums[j] == target - nums[i]:
                return [i, j]


def two_sum_two_pass_hash(nums: List[int], target: int) -> List[int]:
    """
    Returns the indices of two numbers that add up to a specific target

    Arguments

    * `nums` - A vector of integers
    * `target` - Result of the sum of two numbers in `nums`

    Approach

    This solution converts the `nums` vector into a HashMap
    and iterates through the `nums` vector` again to test which
    number has a complement.
    """
    num_map = {}
    for i, num in enumerate(nums):
        num_map[num] = i

    for i, num in enumerate(nums):
        complement = target - num
        if complement in num_map and num_map[complement] != i:
            return [i, num_map[complement]]


def two_sum_one_pass_hash(nums: List[int], target: int) -> List[int]:
    """
    Returns the indices of two numbers that add up to a specific target
    
    Arguments
    
    * `nums` - A vector of integers
    * `target` - Result of the sum of two numbers in `nums`
    
    Approach
    
    This solution uses a HashMap to store the complement - `target` - `num`.
    If a complement is seen in `nums` then return a vector containing the current index
    and the complements index.
    """

    complements = {}
    for i, num in enumerate(nums):
        if num in complements:
            return [complements[num], i]
        else:
            complements[target - num] = i


def test_example_1():
    nums = [2, 7, 11, 15]
    target = 9
    assert two_sum_brute_force(nums, target) == [0, 1]
    assert two_sum_two_pass_hash(nums, target) == [0, 1]
    assert two_sum_one_pass_hash(nums, target) == [0, 1]
