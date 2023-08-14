/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

/// Groups anagrams from a given list of strings.
///
/// Anagrams are words that have the same characters but in a different order.
/// This function takes a vector of strings as input and returns a vector of vectors of strings,
/// where each inner vector contains strings that are anagrams of each other.
///
/// Time complexity: O(n * m), where n is the number of strings and m is the maximum length of a string.
///     - n is the number of strings, so we iterate through each string in the input vector in the outer loop.
///     - m is the maximum length of a string, so for each string, we iterate through each character in the inner loop.
///     - Overall, we have O(n * m) time complexity.
/// Space complexity: O(n * m), where n is the number of strings and m is the maximum length of a string.
///     - We use a HashMap to store the anagram groups, which can have at most n keys (one for each string).
///     - For each key, we store a vector of strings, which can have at most n * m elements.
///     - Therefore, the space complexity is O(n * m).

// @lc code=start
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // Create a HashMap to store the anagram groups.
        // The key is a fixed-size array of 26 bytes representing the count of each character in a string.
        // The value is a vector of strings, which are anagrams of each other.
        let mut result = std::collections::HashMap::<[u8; 26], Vec<String>>::new();

        // Iterate through each string in the input vector.
        for s in strs {
            // Create a fixed-size array to store the count of each character in the current string.
            // The array has a size of 26, which corresponds to the 26 lowercase English letters.
            let mut key: [u8; 26] = [0; 26];

            for c in s.chars() {
                // Convert the character to its ASCII value and subtract the ASCII value of 'a'
                // to get the index of the character in the key array.
                // Increment the count of the character in the key array.
                key[c as usize - 'a' as usize] += 1;
            }

            // Insert the current string into the corresponding anagram group in the HashMap.
            // If the key doesn't exist in the HashMap, create a new empty vector as the value.
            // Then, push the current string into the vector.
            result.entry(key).or_insert(Vec::new()).push(s);
        }

        // Convert the values of the HashMap into a vector and return it.
        // This discards the keys and only keeps the anagram groups.
        result.into_values().collect()
    }
}
// @lc code=end
