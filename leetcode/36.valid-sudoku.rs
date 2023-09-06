/*
 * @lc app=leetcode id=36 lang=rust
 *
 * [36] Valid Sudoku
 */

// @lc code=start
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // check rows for 1-9
        for row in &board {
            let mut set = std::collections::HashSet::new();

            if !row
                .iter()
                .all(|&c| if c == '.' { true } else { set.insert(c) })
            {
                return false;
            }
        }

        // check cols for 1-9
        for j in 0..9 {
            let mut set = std::collections::HashSet::new();

            for row in &board {
                if row[j] != '.' {
                    if !set.insert(row[j]) {
                        return false;
                    }
                }
            }
        }

        // check in 3*3 matrix
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
            }
        }

        // return true
        true
    }
}
// @lc code=end
