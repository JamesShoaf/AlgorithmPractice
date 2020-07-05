class Solution:
    def hamming_distance(self, x: int, y: int) -> int:
        if (not isinstance(x, int) or not isinstance(y, int)
            or isinstance(x, bool) or isinstance(y, bool)) : return -1
        xor = x ^ y
        counter = 0
        if xor > 0:
            # this loop works in fewer steps for positive numbers, but fails for negative numbers
            while xor:
                counter += 1
                xor &= (xor - 1)
        elif xor < 0:
            for i in range(32):
                current_bit = 1 << i
                if xor & current_bit: counter += 1
        return counter