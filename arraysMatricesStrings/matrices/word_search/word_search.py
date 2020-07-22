from typing import List

class Solution():
    def word_search(self, board: List[List[str]], word: str) -> bool:
        word_length = len(word)
        if word_length == 0: return True
        row_count = len(board)
        if row_count == 0: return False
        col_count = len(board[0])
        if col_count == 0: return False
        adjacencies = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        visited = set()
        def recursive_search(row: int, col: int, index: int) -> bool:
            if index >= word_length: return True
            if row < 0 or row >= row_count or col < 0 or col >= col_count:
                return False
            if (row, col) in visited: return False
            if board[row][col] != word[index]: return False
            visited.add((row, col))
            for row_adj, col_adj in adjacencies:
                if recursive_search(row + row_adj, col + col_adj, index + 1):
                    return True
            visited.remove((row, col))
            return False

        for row_index, row in enumerate(board):
            for col_index, char in enumerate(row):
                if char == word[0]:
                    if recursive_search(row_index, col_index, 0): return True
        return False