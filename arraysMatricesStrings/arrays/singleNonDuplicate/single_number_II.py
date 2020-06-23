from typing import List
import math
import functools
# Given a non-empty array of integers, every element appears three times except for one, which
# appears exactly once. Find that single one.

# O(n) time and O(1) space complexity
class Solution:
    def singleNumber(self, nums:List[int]) -> int:
        # find the maximum value in the array and determine whether the single is negative
        max_val = -1
        neg_count = 0
        for i in range(len(nums)):
            if nums[i] < 0: neg_count += 1
            if abs(nums[i]) > max_val: max_val = abs(nums[i])
        # find the maximum power of two to iterate through
        most_significant_bit = math.floor(math.log2(max_val))
        output = 0
        # for each bit position
        for i in range(most_significant_bit + 1):
            # 2 ** i = 1<<i for natural i values
            # count the number of 1s that appear at that position
            if functools.reduce(lambda a,b: a + int(bool(abs(b)&1<<i)), nums, 0) % 3:
                # and toggle that bit if there are 3y + 1 elements with that bit
                output |= 1<<i
        # swap the sign if there are 3y + 1 odd elements
        if neg_count % 3: output *= -1
        return output