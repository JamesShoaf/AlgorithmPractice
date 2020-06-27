class Solution:
    def numSquares(self, n:int) -> int:
        try:
            if not isinstance(n, int): raise TypeError
            memo = {0: 0}
            squares = []
            i = 2
            square = 4
            while square <= n:
                squares.append(square)
                i += 1
                square = i ** 2
            def count_squares(num:int) -> int:
                # if a negative integer is passed in, raise an error
                if num < 0: raise ValueError
                if memo.get(num) != None: return memo[num]
                minimum_squares = memo[num - 1]
                for i in range(len(squares)):
                    # stop checking squares if num is a perfect square
                    if minimum_squares == 0: break
                    difference = num - squares[i]
                    # if the difference is negative, don't continue for larger squares
                    if difference >= 0:
                        minimum_squares = min(minimum_squares, memo[difference])
                    else: break
                memo[num] = minimum_squares + 1
                return memo[num]
            # build up solutions to subproblems from 0 to n
            for i in range(n):
                count_squares(i)
            return count_squares(n)
        except: return -1