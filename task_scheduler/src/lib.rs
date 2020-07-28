use std::cmp;

/// Calculates the smallest interval to execute the given tasks
/// given a cooldown period between the same tasks
/// 
/// # Arguments
/// 
/// * `tasks` - The list of tasks to be executed
/// * `n` - The cool down period between the same tasks
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    // Populate frequency counts
    let mut task_frequency = vec![0; 26];

    for &task in &tasks {
        task_frequency[task as usize - 'A' as usize] += 1;
    }

    // Sort and calculate max frequency and idle count
    // max frequency is -1 since the first instance doesn't count
    task_frequency.sort_unstable_by(|a, b| b.cmp(a));
    let max_frequency = task_frequency[0] - 1;
    let mut idle_count = max_frequency * n;

    // Fill in the remaining tasks between the task
    // with the maximum frequency
    for &frequency in task_frequency.iter().skip(1) {
        idle_count -= cmp::min(max_frequency, frequency);
    }

    // At minimum, the interval is the number of task
    // If idle count is greater than 0 then it's 
    // idle_count + number of tasks
    if idle_count > 0 {
        idle_count + tasks.len() as i32
    } else {
        tasks.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 2;
        assert_eq!(least_interval(tasks, n), 8);
    }

    #[test]
    fn test_example_2() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 0;
        assert_eq!(least_interval(tasks, n), 6);
    }

    #[test]
    fn test_example_3() {
        let tasks = vec!['A','A','A','A','A','A','B','C','D','E','F','G'];
        let n = 2;
        assert_eq!(least_interval(tasks, n), 16);
    }
}
