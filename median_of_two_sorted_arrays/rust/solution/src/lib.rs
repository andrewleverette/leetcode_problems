use std::cmp;

/// Returns the median of two sorted arrays
/// 
/// # Arguments
/// 
/// * `nums1` - A vector of sorted numbers
/// * `nums2` - A vector of sorted numbers
/// 
/// # Approach
/// 
/// Merge both vectors together by iterating over both input vectors
/// and placing items in the merged vector in order.
pub fn find_median_sorted_arrays_brute_force(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let length = nums1.len() + nums2.len();
    let mut merged_array = Vec::with_capacity(length);
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() || j < nums2.len() {
        if i == nums1.len() {
            merged_array.push(nums2[j]);
            j += 1;
        } else if j == nums2.len() {
            merged_array.push(nums1[i]);
            i += 1;
        } else {
            if nums1[i] <= nums2[j] {
                merged_array.push(nums1[i]);
                i += 1;
            } else {
                merged_array.push(nums2[j]);
                j += 1;
            }
        }
    }
    
    let mid = length / 2;
    if length % 2 == 0 {
        (merged_array[mid] + merged_array[mid - 1]) as f64 / 2.0
    } else {
        merged_array[mid] as f64
    }
}

/// Returns the median of two sorted arrays
/// 
/// # Arguments
/// 
/// * `nums1` - A vector of sorted numbers
/// * `nums2` - A vector of sorted numbers
/// 
/// # Approach
///
/// Partition each vector into halves such that each half so that all
/// element in the first half are less than all the values in teh second half.
pub fn find_median_sorted_arrays_intuitively(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() <= nums2.len() {
        find_median(nums1, nums2)
    } else {
        find_median(nums2, nums1)
    }
}

fn find_median(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {    
    let n = nums1.len();
    let m = nums2.len();
    let mut median = 0;
    let mut min = 0;
    let mut max = n;
    let mut i = 0;
    let mut j = 0;

    while min <= max {
        i = (min + max) / 2;
        j = (n + m + 1) / 2 - i;

        if i < n && j > 0 && nums2[j - 1] > nums1[i] {
            // First half is not empty but the last value in the
            // first half of nums2 is greater than the first value in the second
            // half of nums1
            min = i + 1;
        } else if i > 0 && j < m && nums2[j] < nums1[i - 1] {
            // Second half is not empty but the first value in the second
            // half of nums2 is less than the last value in the first half
            // of nums1
            max = i - 1;
        } else {
            // Numbers are partitioned correctly
            if i == 0 {
                // First half of nums1 is an empty set
                median = nums2[j - 1];
            } else if j == 0 {
                // First half of nums2 is an empty set
                median = nums1[i - 1];
            } else {
                median = cmp::max(nums1[i - 1], nums2[j - 1]);
            }
            break;
        }
    }

    if (n + m) % 2 == 1 {
        median as f64
    } else if i == n {
        // Second half of nums1 is empty
        (median + nums2[j]) as f64 / 2.0
    } else if j == m {
        // Second half of nums2 is empty
        (median + nums1[i]) as f64 / 2.0
    } else {
        (median + cmp::min(nums1[i], nums2[j])) as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays_brute_force(nums1.clone(), nums2.clone()), 2.0);
        assert_eq!(find_median_sorted_arrays_intuitively(nums1, nums2), 2.0);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays_brute_force(nums1.clone(), nums2.clone()), 2.5);
        assert_eq!(find_median_sorted_arrays_intuitively(nums1, nums2), 2.5);
    }

    #[test]
    fn test_example_3() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(find_median_sorted_arrays_brute_force(nums1.clone(), nums2.clone()), 1.0);
        assert_eq!(find_median_sorted_arrays_intuitively(nums1, nums2), 1.0);
    }

    #[test]
    fn test_example_4() {
        let nums1 = vec![];
        let nums2 = vec![2, 3];
        assert_eq!(find_median_sorted_arrays_brute_force(nums1.clone(), nums2.clone()), 2.5);
        assert_eq!(find_median_sorted_arrays_intuitively(nums1, nums2), 2.5);
    }

    #[test]
    fn test_example_5() {
        let nums1 = vec![1];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays_brute_force(nums1.clone(), nums2.clone()), 1.0);
        assert_eq!(find_median_sorted_arrays_intuitively(nums1, nums2), 1.0);
    }

    #[test]
    fn test_example_6() {
        let nums1 = vec![2, 3];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays_brute_force(nums1.clone(), nums2.clone()), 2.5);
        assert_eq!(find_median_sorted_arrays_intuitively(nums1, nums2), 2.5);
    }
}
