/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

///
/// The overall space complexity of the program is O(n),
/// where n is the number of elements in the nums vector.
/// This is because the program uses a hashmap to store the complement of each number and its index,
/// and the space required for the hashmap is proportional to the number of elements in nums.
/// The overall time complexity of the program is O(n),
/// where n is the number of elements in the nums vector.
/// This is because the program iterates over the numbers in nums once,
/// performing constant-time operations (such as hashmap lookups and insertions) for each number.
///

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a hashmap to store the complement of each number and its index
        let mut map = std::collections::HashMap::<i32, i32>::with_capacity(nums.len()); // O(n) space complexity

        // Iterate over the numbers and their indices
        // O(n) time complexity
        for (index, num) in nums.iter().enumerate() {
            // Calculate the complement of the current number
            let complement = target - num; // O(1) time complexity

            // Check if the complement exists in the hashmap
            if map.contains_key(&complement) {
                // O(1) time complexity
                // If found, return the indices of the two numbers
                return vec![*map.get(&complement).unwrap(), index as i32]; // O(1) time complexity
            }

            // Insert the current number and its index into the hashmap
            map.insert(*num, index as i32); // O(1) time complexity
        }

        // If no two numbers are found, return an empty vector
        vec![] // O(1) space complexity
    }
}
// @lc code=end
