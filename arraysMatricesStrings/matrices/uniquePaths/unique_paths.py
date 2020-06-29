class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        if (not isinstance(m, int) or not isinstance(n, int)
            or m <= 0 or n <= 0): return None
        if m == 1 or n == 1: return 1
        smaller, bigger = (m, n) if m <= n else (n, m)
        dp = [[1 for x in range(smaller)] for y in range(2)]
        for row in range(1, bigger):
            for col in range(1, smaller):
                dp[row % 2][col] = dp[(row - 1) % 2][col] + dp[row % 2][col - 1]
        return dp[(bigger - 1) % 2][smaller - 1]