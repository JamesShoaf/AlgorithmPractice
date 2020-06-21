from typing import List
import math

class Solution:
    def solve(self, citations: List[int]) -> int:
        N = len(citations)
        low = 0
        high = N - 1
        # edge case: if there are no papers or all papers have 0 or fewer citations, return 0
        if N < 1 or citations[high] < 1: return 0
        while(True):
            mid = math.floor((low + high) / 2)
            mid_val = citations[mid]
            vals_mid_or_greater = N - mid
            if mid_val == vals_mid_or_greater: return vals_mid_or_greater
            if mid_val < vals_mid_or_greater:
                if mid == low: return N - high
                low = mid
            if mid_val > vals_mid_or_greater:
                if mid == low: return N - low
                high = mid
