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
/// This solution uses a HashMap to store the complement - `target` - `num`.
/// If a complement is seen in `nums` then return a vector containing the current index
/// and the complements index.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
    fn test_given_example() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
