from typing import List

class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        if len(s) == 0: return []
        dp = [[] for _ in range(len(s))]
        word_set = set(wordDict)
        char_set = set(''.join(wordDict))
        for char in s:
            if char not in char_set:
                return []
        for i in range(len(s)):
            for j in range(i, -1, -1):
                current_slice = s[j: i+1]
                if current_slice in word_set:
                    if j == 0:
                        dp[i].append(current_slice)
                    else:
                        for string in dp[j - 1]:
                            dp[i].append(f'{string} {current_slice}')
        return dp[len(s) - 1]