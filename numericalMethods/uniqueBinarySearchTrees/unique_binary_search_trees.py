class Solution:
    memo = {0: 1}
    def numTrees(self, n: int) -> int:
        if not isinstance(n, int): return 0
        if self.memo.get(n): return self.memo.get(n)
        if n < 0: return 1
        count = 0
        for i in range(n):
            count += self.numTrees(i) * self.numTrees(n - i - 1)
        self.memo[n] = count
        return count