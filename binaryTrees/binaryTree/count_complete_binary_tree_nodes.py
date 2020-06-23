# Given a complete binary tree, count the number of nodes.

# Note: In a complete binary tree every level, except possibly the last, is completely filled,
# and all nodes in the last level are as far left as possible. It can have between 1 and 2h
# nodes inclusive at the last level h.

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def countNodes(self, root: TreeNode) -> int:
        # return 0 if passed an empty tree
        if not root: return 0
        # find the height of the leftmost node
        left = root
        max_height = 1
        while (left.left):
            left = left.left
            max_height += 1
        # calculate the theoretical maximum number of nodes
        node_count = (1<<max_height) - 1
        # reverse in-order traversal to find the rightmost node at the leftmost's height
        # initialize empty stack, pointer to current node, and current_height to 1
        stack = []
        current = root
        current_height = 1
        # loop until the first node at the max_height is found
        while not current or current_height < max_height:
            # push nodes and their heights to the stack and set current rightward until current is None
            if current:
                stack.append((current, current_height))
                current = current.right
                current_height += 1
            # when the current value is None, pop the last node and height (the parent) off the stack
            else:
                current, current_height = stack.pop()
                # if that node is at the height before the max, that node is missing one or both children
                # decrement the node count by one if only right is missing, or two if both are missing
                if current_height == max_height - 1: 
                    if current.left: node_count -= 1
                    else: node_count -= 2
                # if the left child is missing, the top of the stack will be the parent node
                # if the left child is the target, the loop will end there
                current = current.left
                current_height += 1
        return node_count