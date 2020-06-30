from typing import List
from context import Trie

class Solution:
    def search(self, board: List[List[str]], words: List[str]) -> List[str]:
        # construct a trie from the word list
        row_count = len(board)
        if not row_count: return []
        col_count = len(board[0])
        if not col_count: return []
        words_trie = Trie()
        for word in words:
            words_trie.insert(word)
        adjacencies = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        char_stack = []
        coord_set = set()
        output_set = set()
        def recursive_search (row: int, col: int, parent = words_trie) -> None:
            char = board[row][col]
            current = parent.starts_with(char)
            if current:
                char_stack.append(char)
                coords = (row, col)
                if current.end: output_set.add(''.join(char_stack))
                coord_set.add(coords)
                for row_adj, col_adj in adjacencies:
                    new_row = row + row_adj
                    new_col = col + col_adj
                    if ((new_row, new_col) not in coord_set and new_row >= 0
                        and new_row < row_count and new_col >= 0
                        and new_col < col_count):
                        recursive_search(new_row, new_col, current)
                char_stack.pop()
                coord_set.remove(coords)
        for row in range(row_count):
            for col in range(col_count):
                recursive_search(row, col)
        return list(output_set)