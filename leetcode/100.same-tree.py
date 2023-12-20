#
# @lc app=leetcode id=100 lang=python3
#
# [100] Same Tree
#

# Definition for a binary tree node.


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


from typing import Optional


# @lc code=start
class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if p is None and q is None:
            # p, q is None -> Identical
            return True

        if p is None or q is None:
            # p or q is not None; hence not identical
            return False

        return (
            # same value => identical node
            p.val == q.val
            # if same values is same position => identical
            and self.isSameTree(p.left, q.left)
            and self.isSameTree(p.right, q.right)
        )


# @lc code=end
