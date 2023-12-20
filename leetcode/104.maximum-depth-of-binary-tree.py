#
# @lc app=leetcode id=104 lang=python3
#
# [104] Maximum Depth of Binary Tree
#


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# @lc code=start
from typing import Optional, List


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        # base case; if root node does not exists
        if not root:
            return 0

        # stack to keep track of nodes, current depth; max
        stack: List[tuple[TreeNode, int]] = [(root, 1)]
        max_depth: int = 1

        # do dfs; update stack;
        # compare max and current depth --> store max depth
        while len(stack) > 0:
            # cursor
            current, depth = stack.pop()

            # traverse
            if current.left is not None:
                stack.append((current.left, depth + 1))
            if current.right is not None:
                stack.append((current.right, depth + 1))

            # update max_depth
            max_depth = max(depth, max_depth)

        return max_depth


# @lc code=end
