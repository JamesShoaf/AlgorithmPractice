# given an array of integers in which each integer is contained twice, except for two which
# are contained once, return the two integers which are contained only once

from typing import List
class Solution:
    def singleNumberIII(self, nums:List[int]) -> List[int]:
        # xor the entire list. This returns the xor of the two single numbers
        # (x ^ x = 0 and x ^ 0 = x)
        xor = 0
        for num in nums: xor ^= num
        # because of the way that negative numbers are stored (-1 = 111...1),
        # bitwise and of a number and itself returns the least significant bit set to 1
        low_bit = xor & -xor
        # this bit can be used as a mask to separate the array into two groups
        xor_with_low_bit = 0
        xor_without_low_bit = 0
        for num in nums:
            # those that and with the bit
            if num & low_bit:
                xor_with_low_bit ^= num
            # and those which do not
            else:
                xor_without_low_bit ^= num
        # xor of these groups returns the single number from each of them,
        # though order is not guaranteed
        return [xor_with_low_bit, xor_without_low_bit]