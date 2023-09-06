/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // create a new hash set
        let mut set = std::collections::HashSet::new();

        // return true if set.insert fails else true
        !nums.iter().all(|&num| set.insert(num))
    }
}
// @lc code=end
