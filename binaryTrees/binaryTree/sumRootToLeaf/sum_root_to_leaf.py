from context import TreeNode

# In-order traversal
# initialize count and stack
class Solution:
    def sum_root_to_leaf(self, root:TreeNode) -> int:
        try:
            count = 0
            current = root
            val = 0
            stack = []
            # push the node and its adjusted value to the stack
            # adjusted value = parent's adjusted value * 10 + node's value
            while current or len(stack):
                if current:
                    while current:
                        if not isinstance(current.val, int): raise TypeError
                        val = val * 10 + current.val
                        stack.append((current, val))
                        current = current.left
                else:
                    current, val = stack.pop()
                    if not current.left and not current.right:
                        count += val
                    current = current.right
            return count
        except: return None

        