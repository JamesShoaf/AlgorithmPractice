import math
from typing import List
class Matrix_Cell:
    def __init__(self, ldd, ldh, lhd, lhh):
        self.least_damage_damage = ldd
        self.least_damage_hp = ldh
        self.least_hp_damage = lhd
        self.least_hp_hp = lhh

class Solution():
    def calculateMinimumHP(self, dungeon: List[List[int]]) -> int:
        row_count = len(dungeon)
        if row_count == 0: return 1
        col_count = len(dungeon[0])
        if col_count == 0: return 1
        # each cell of dp array contains struct which tracks the two paths to each cell which
        # incur either the least damage or the least HP requirement
        # initialize the board with infinite minimum HP/ infinite damage to avoid going out of bounds
        # except the cell below the starting square, which initializes to 1 hp and 0 damage
        dp = [[Matrix_Cell(math.inf, math.inf, math.inf, math.inf) for x in range(col_count)] for y in range(2)]
        dp[1][0] = Matrix_Cell(0, 1, 0, 1)
        for row in range(row_count):
            for col in range(col_count):
                room_damage = dungeon[row][col]
                current = dp[row % 2][col]
                above = dp[(row - 1) % 2][col]
                ldd_above = above.least_damage_damage - room_damage
                ldh_above = max(above.least_damage_hp, ldd_above + 1)
                lhd_above = above.least_hp_damage - room_damage
                lhh_above = max(above.least_hp_hp, lhd_above + 1)

                least_damage_above = (
                    (ldd_above, ldh_above) if ldd_above < lhd_above
                    else (lhd_above, lhh_above))
                least_hp_above = (
                    (lhd_above, lhh_above) if lhh_above < ldh_above
                    else (ldd_above, ldh_above))

                if col == 0:
                    current.least_damage_damage, current.least_damage_hp = least_damage_above
                    current.least_hp_damage, current.least_hp_hp = least_hp_above
                else:
                    left = dp[row % 2][col - 1]
                    ldd_left = left.least_damage_damage - room_damage
                    ldh_left = max(left.least_damage_hp, ldd_left + 1)
                    lhd_left = left.least_hp_damage - room_damage
                    lhh_left = max(left.least_hp_hp, lhd_left + 1)

                    least_damage_left = (
                        (ldd_left, ldh_left) if ldd_left < lhd_left else (lhd_left, lhh_left))
                    least_hp_left = ((lhd_left, lhh_left) if lhh_left < ldh_left
                        else (ldd_left, ldh_left))

                    if least_damage_above[0] == least_damage_left[0]:
                        current.least_damage_damage, current.least_damage_hp = (
                            least_damage_above if least_damage_above[1] < least_damage_left[1]
                            else least_damage_left)
                    else: current.least_damage_damage, current.least_damage_hp = (
                        least_damage_above if least_damage_above[0] < least_damage_left[0]
                        else least_damage_left)

                    if least_hp_above[1] == least_hp_left[1]:
                        current.least_hp_damage, current.least_hp_hp = (
                            least_hp_above if least_hp_above[0] < least_hp_left[0]
                            else least_hp_left)
                    else: current.least_hp_damage, current.least_hp_hp = (
                        least_hp_above if least_hp_above[1] < least_hp_left[1]
                        else least_hp_left)

        return dp[(row_count - 1) % 2][col_count - 1].least_hp_hp