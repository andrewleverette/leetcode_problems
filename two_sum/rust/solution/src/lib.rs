use std::collections::HashMap;

/// Returns the indices of two numbers that add up to a specific target
///
/// # Arguments
///
/// * `nums` - A vector of integers
/// * `target` - Result of the sum of two numbers in `nums`
///
/// # Approach
///
/// This naive solution loops through each element `x` in `nums` and 
/// then loops through each element again to find if there is a value 
/// that equals `target` - `x`.
pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[j] == target - nums[i] {
                return vec![i as i32, j as i32]
            }
        }
    }

    vec![]
}

/// Returns the indices of two numbers that add up to a specific target
///
/// # Arguments
///
/// * `nums` - A vector of integers
/// * `target` - Result of the sum of two numbers in `nums`
///
/// # Approach
///
/// This solution converts the `nums` vector into a HashMap
/// and iterates through the `nums` vector` again to test which
/// number has a complement.
pub fn two_sum_two_pass_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        num_map.insert(*num, i as i32);
    }

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        
        if let Some(&index) = num_map.get(&complement) {
            if index != i as i32 {
                return vec![i as i32, index];
            }
        }
    }

    vec![]
}

/// Returns the indices of two numbers that add up to a specific target
///
/// # Arguments
///
/// * `nums` - A vector of integers
/// * `target` - Result of the sum of two numbers in `nums`
///
/// # Approach
///
/// This solution uses a HashMap to store the complement - `target` - `num`.
/// If a complement is seen in `nums` then return a vector containing the current index
/// and the complements index.
pub fn two_sum_one_pass_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match complements.get(num) {
            Some(&index) => return vec![index, i as i32],
            None => complements.insert(target - num, i as i32),
        };
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum_brute_force(nums.clone(), target), vec![0, 1]);
        assert_eq!(two_sum_two_pass_hash(nums.clone(), target), vec![0, 1]);
        assert_eq!(two_sum_one_pass_hash(nums, target), vec![0, 1]);
    }
}
