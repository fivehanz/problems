/*
 * @lc app=leetcode id=242 lang=rust
 *  TC: O(nlogn), SC: O(n)
 * [242] Valid Anagram
 */

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // sanity checks
        if s.len() != t.len() {
            return false;
        }

        // convert String to &[u8] bytes then to mutable Vec
        let mut s_as_bytes: Vec<u8> = s.as_bytes().to_vec(); // O(n)
        let mut t_as_bytes: Vec<u8> = t.as_bytes().to_vec(); // O(n)

        // sort the bytes in place
        s_as_bytes.sort(); // O(nlogn)
        t_as_bytes.sort(); // O(nlogn)

        // check if the sorted bytes are equal
        s_as_bytes == t_as_bytes // O(n)
    }
}
// @lc code=end
