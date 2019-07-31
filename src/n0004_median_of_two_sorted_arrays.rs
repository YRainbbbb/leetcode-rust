/**
 * [4] Median of Two Sorted Arrays
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 * 
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 * 
 * You may assume nums1 and nums2 cannot be both empty.
 * 
 * Example 1:
 * 
 *
 * nums1 = [1, 3]
 * nums2 = [2]
 * 
 * The median is 2.0
 * 
 * 
 * Example 2:
 * 
 * 
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 * 
 * The median is (2 + 3)/2 = 2.5
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

// TODO: nth slice
use std::rc::Rc;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (lhs,rhs) = if nums1.len() >nums2.len(){(Rc::new(nums2),Rc::new(nums1))}else{(Rc::new(nums1),Rc::new(nums2))};
        let (n,m) = (Rc::clone(&lhs).len(),Rc::clone(&rhs).len());
        let (mut lmax1,mut rmin1,mut lmax2,mut rmin2) = (0,0,0,0);
        let (mut lo,mut hi) = (0,n*2);
        while lo <= hi{
            let c1 = (lo+hi)/2;
            let c2 = m+n-c1;
            lmax1 = if c1==0{i32::min_value()}else{Rc::clone(&lhs)[(c1-1)/2]};
            rmin1 = if c1==2*n{i32::max_value()}else{Rc::clone(&lhs)[c1/2]};
            lmax2 = if c2==0{i32::min_value()}else{Rc::clone(&rhs)[(c2-1)/2]};
            rmin2 = if c2==2*m{i32::max_value()}else{Rc::clone(&rhs)[c2/2]};

        if lmax1>rmin2{hi=c1-1}
        else if lmax2>rmin1{lo = c1+1}
        else{break}
    }
    (std::cmp::max(lmax1, lmax2) + std::cmp::min(rmin1, rmin2))as f64 / 2.0
}
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: implementation
    #[test]
    #[ignore]
    fn test_4() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
