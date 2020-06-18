from typing import List

class Solution:
    def solve(self, board: List[List[str]]) -> None:
        rows = len(board)
        cols = len(board[0])
        visited = set()
        current_visit = set()
        def is_surrounded(row: int, col: int) -> bool:
            if row < 0 or row > rows - 1: return False
            if col < 0 or col > cols - 1: return False
            if (row, col) in current_visit: return True
            if board[row][col] == 'X': return True
            visited.add((row, col))
            current_visit.add((row, col))
            if (is_surrounded(row - 1, col) and is_surrounded(row, col - 1) and
                    is_surrounded(row + 1, col) and is_surrounded(row, col + 1)):
                return True
            return False
        def toggle(row: int, col: int) -> None:
            if row < 0 or row > rows - 1: return None
            if col < 0 or col > cols - 1: return None
            if board[row][col] == 'X': return None
            board[row][col] = 'X'
            toggle(row - 1, col)
            toggle(row, col - 1)
            toggle(row + 1, col)
            toggle(row, col + 1)
        for row, rowList in enumerate(board):
            for col in range(len(rowList)):
                current_visit = set()
                if ((row, col) not in visited and board[row][col] == 'O' and
                        is_surrounded(row, col)):
                    toggle(row, col)
