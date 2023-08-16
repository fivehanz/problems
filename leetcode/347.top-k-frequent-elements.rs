/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
impl Solution {
    /// Returns the top k frequent numbers in the given vector.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers.
    /// * `k` - The number of top frequent numbers to return.
    ///
    /// # Example
    ///
    /// ```
    /// let nums = vec![1, 1, 1, 2, 2, 3];
    /// let k = 2;
    /// let result = top_k_frequent(nums, k);
    /// println!("{:?}", result); // Output: [1, 2]
    /// ```
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = std::collections::HashMap::<i32, i32>::new();

        // Initialize a vector of vectors to store numbers based on their frequency.
        let mut freq: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

        // Count the frequency of each number using a hashmap.
        for n in nums {
            count.entry(n).and_modify(|c| *c += 1).or_insert(1);
        }

        // Store numbers in the freq vector based on their frequency.
        for (n, c) in count.iter() {
            freq[*c as usize].push(*n as i32);
        }

        // Build the result by iterating over the freq vector in reverse order.
        let mut res: Vec<i32> = vec![];
        for i in (0..freq.len()).rev() {
            // Iterate over the numbers in the current frequency.
            for n in freq[i].iter() {
                res.push(*n);
                // Return the result if we have collected k numbers.
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
    }
}
// @lc code=end
