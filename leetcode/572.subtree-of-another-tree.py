#
# @lc app=leetcode id=572 lang=python3
#
# [572] Subtree of Another Tree
#


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val: int | None = val
        self.left: TreeNode | None = left
        self.right: TreeNode | None = right


# @lc code=start
from typing import Optional


class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        if subRoot is None:
            # empty subtree is always a subtree
            return True
        if root is None:
            # empty root does not have a subtree
            return False

        if self.isIdentical(root, subRoot):
            return True

        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)

    def isIdentical(self, r: Optional[TreeNode], s: Optional[TreeNode]) -> bool:
        if r is None or s is None:
            if r is None and s is None:
                # root and subtree are None => identical
                return True
            # one of them is not None => not identical
            return False

        if r.val == s.val:
            return self.isIdentical(r.left, s.left) and self.isIdentical(
                r.right, s.right
            )

        return False


# @lc code=end
