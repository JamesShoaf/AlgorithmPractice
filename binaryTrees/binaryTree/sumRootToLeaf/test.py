import unittest
from sum_root_to_leaf import Solution
from context import TreeNode

class TestSumRootToLeaf(unittest.TestCase):
    solver = Solution()
    def test_invalid_types(self):
        invalid_trees = [
            None,
            1,
            'a',
            TreeNode('a'),
            TreeNode(1, TreeNode('a'), TreeNode(2))
        ]
        for tree in invalid_trees:
            self.assertRaises(TypeError, self.solver.sum_root_to_leaf(tree))
    
    
    def test_non_integer_values(self):
        invalid_trees = [
            TreeNode('a'),
            TreeNode(1, TreeNode('a'), TreeNode(2))
        ]
        for tree in invalid_trees:
            self.assertRaises(TypeError, self.solver.sum_root_to_leaf(tree))

    def test_integer_values(self):
        test_tuples = [
            (TreeNode(1, TreeNode(1, TreeNode(1)), TreeNode(2)), 123),
            (TreeNode(4, TreeNode(1), TreeNode(2, TreeNode(1))), 462),
        ]
        for tree, expected in test_tuples:
            self.assertEqual(self.solver.sum_root_to_leaf(tree), expected)

if __name__ == '__main__':
    unittest.main()