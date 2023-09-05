/*
* @lc app=leetcode id=238 lang=rust
*
* [238] Product of Array Except Self
*/

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // create a vec with capacity N to store the result values
        let mut result: Vec<i32> = Vec::with_capacity(nums.len());

        // loop through nums vec -- forward pass with 1 as prefix default val
        nums.iter().enumerate().fold(1, |acc, (index, &num)| {
            result.push(acc);
            acc * num
        });

        // loop through nums vec -- backward pass with 1 as postfix default val
        nums.iter().enumerate().rev().fold(1, |acc, (index, &num)| {
            result[index] *= acc;
            acc * num
        });

        // return the result
        result
    }
}
// @lc code=end
