pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test_example_3() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn test_example_4() {
        let nums1 = vec![];
        let nums2 = vec![2, 3];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }

}
