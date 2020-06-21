import math

# Given a string S, consider all duplicated substrings: (contiguous) substrings of S that occur 2
# or more times.  (The occurrences may overlap.)
# Return any duplicated substring that has the longest possible length.  (If S does not have a
# duplicated substring, the answer is "".)

class Solution:
    def longestDupSubstring(self, S: str) -> str:
        # the choice of a prime number to perform module operations over is arbitrary as long
        # as it is relatively large. 104729 is the 10000th prime
        prime = 104729
        # char_vals = {'a': 1, 'b': 2, ...}
        char_vals = {}
        abecedery = 'abcdefghijklmnopqrstuvwxyz'
        for index, char in enumerate(abecedery, start=1):
            char_vals[char] = index
        def generate_hash_of_length(length: int) -> int:
            hash_val = 0
            exponent = 1
            # each character is assigned a value and a power of 26
            # 'abc' => 1 * 26^2 + 2 * 26^1 + 3 * 26^0
            for i in range(length):
                hash_val *= 26
                hash_val += char_vals[S[i]]
                hash_val %= prime
            for i in range(length - 1):
                exponent *= 26
                exponent %= prime
            return (hash_val, exponent)
        S_length = len(S)
        # binary search values between 1 and length - 1 for a duplicate string at that length
        low = 1
        high = S_length - 1
        longest_length = 0
        longest_start = 0
        while low <= high:
            mid = math.floor((high + low) / 2)
            hash_indexes = {}
            # hash of length mid starting at index 0
            (current_hash, current_exponent) = generate_hash_of_length(mid)
            hash_indexes[current_hash] = [0]
            substring_match = False
            for i in range(mid, S_length):
                if substring_match: break
                # remove the value of the first character from the hash
                current_hash -= current_exponent * char_vals[S[i - mid]]
                # shift all elements of the current hash up one place
                current_hash *= 26
                # add the last character to the hash
                current_hash += char_vals[S[i]]
                # and take modulus of the current hash again
                current_hash %= prime
                if current_hash in hash_indexes:
                    for index in hash_indexes[current_hash]:
                        character_match = True
                        for j in range(mid):
                            if S[i - mid + 1 + j] != S[index + j]:
                                character_match = False
                                break
                        if character_match:
                            longest_length = mid
                            longest_start = index
                            substring_match = True
                            break
                else:
                    hash_indexes[current_hash] = [i - mid + 1]
            if substring_match:
                low = mid + 1
            else:
                high = mid - 1
        return S[longest_start:longest_length + longest_start]