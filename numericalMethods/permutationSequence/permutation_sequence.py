import math

class Solution:
    def getPermutation(self, n:int, k: int) -> str:
        chars = []
        for i in range(1, n + 1):
            chars.append(f'{i}')
        permutation_index = k - 1
        output = ''
        for m in range(n, 0, -1):
            factorial = math.factorial(m - 1)
            char_index = 0
            while permutation_index >= factorial:
                permutation_index -= factorial
                char_index += 1
            next_char = chars[char_index]
            chars.remove(next_char)
            output += next_char
        return output

solver = Solution()
solution = solver.getPermutation(4, 3)
print(solution)


