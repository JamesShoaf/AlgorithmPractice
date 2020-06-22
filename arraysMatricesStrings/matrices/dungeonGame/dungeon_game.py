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
                # compare only ldh and lhh paths from above if in the first column
                if col == 0:
                    # determine which route takes less damage
                    if ldd_above < lhd_above:
                        current.least_damage_damage = ldd_above
                        current.least_damage_hp = ldh_above
                    # in case of a tie, the lower hp route wins
                    else:
                        current.least_damage_damage = lhd_above
                        current.least_damage_hp = lhh_above
                    # determine which route takes less max hp
                    if lhh_above < ldh_above:
                        current.least_hp_damage = lhd_above
                        current.least_hp_hp = lhh_above
                    # in case of a tie, the lower damage route wins
                    else:
                        current.least_hp_damage = lhd_above
                        current.least_hp_hp = lhh_above

                # compare all four routes to the cell if not in the first column
                # Overview: Choose a Left path, then choose an Above path, then compare those two paths
                else:
                    left = dp[row % 2][col - 1]
                    ldd_left = left.least_damage_damage - room_damage
                    ldh_left = max(left.least_damage_hp, ldd_left + 1)
                    lhd_left = left.least_hp_damage - room_damage
                    lhh_left = max(left.least_hp_hp, lhd_left + 1)
                    
                    # determine which route from the left takes less damage
                    if ldd_left < lhd_left:
                        #then determine which route from above takes less damage
                        if ldd_above < lhd_above:
                            # then determine if the left or above takes less damage
                            # in case of a tie, sort by hp
                            if ldd_left == ldd_above:
                                if ldh_left < ldh_above:
                                    current.least_damage_damage = ldd_left
                                    current.least_damage_hp = ldh_left
                                else:
                                    current.least_damage_damage = ldd_above
                                    current.least_damage_hp = ldh_above
                            elif ldd_left < ldd_above:
                                current.least_damage_damage = ldd_left
                                current.least_damage_hp = ldh_left
                            else:
                                current.least_damage_damage = ldd_above
                                current.least_damage_hp = ldh_above
                        # lhd_above <= ldd_above
                        else:
                            if ldd_left == lhd_above:
                                if ldh_left < lhh_above:
                                    current.least_damage_damage = ldd_left
                                    current.least_damage_hp = ldh_left
                                else:
                                    current.least_damage_damage = lhd_above
                                    current.least_damage_hp = lhh_above
                            elif ldd_left < lhd_above:
                                current.least_damage_damage = ldd_left
                                current.least_damage_hp = ldh_left
                            else:
                                current.least_damage_damage = lhd_above
                                current.least_damage_hp = lhh_above

                    # lhd_left <= ldd_left - in case of a tie, less health wins
                    else:
                        #then determine which route from above takes less damage
                        if ldd_above < lhd_above:
                            # then determine if the left or above takes less damage
                            # in case of a tie, sort by hp
                            if lhd_left == ldd_above:
                                if lhh_left < ldh_above:
                                    current.least_damage_damage = lhd_left
                                    current.least_damage_hp = lhh_left
                                else:
                                    current.least_damage_damage = ldd_above
                                    current.least_damage_hp = ldh_above
                            if ldd_left < ldd_above:
                                current.least_damage_damage = lhd_left
                                current.least_damage_hp = lhh_left
                            else:
                                current.least_damage_damage = ldd_above
                                current.least_damage_hp = ldh_above

                        # lhd_above <= ldd_above
                        else:
                            if lhd_left == lhd_above:
                                if lhh_left < lhh_above:
                                    current.least_damage_damage = lhd_left
                                    current.least_damage_hp = lhh_left
                                else:
                                    current.least_damage_damage = lhd_above
                                    current.least_damage_hp = lhh_above
                            if lhd_left < lhd_above:
                                current.least_damage_damage = lhd_left
                                current.least_damage_hp = lhh_left
                            else:
                                current.least_damage_damage = lhd_above
                                current.least_damage_hp = lhh_above
                    
                    # determine which route from the left takes less hp
                    if lhh_left < ldh_left:
                        # then determine which route from above takes less hp
                        if lhh_above < ldh_above:
                            # then determine whether left or above takes less hp
                            # in case of a tie, sort by damage
                            if lhh_left == lhh_above:
                                if lhd_left < lhd_above:
                                    current.least_hp_damage = lhd_left
                                    current.least_hp_hp = lhh_left
                                else:
                                    current.least_hp_damage = lhd_above
                                    current.least_hp_hp = lhh_above
                            elif lhh_left < lhh_above:
                                current.least_hp_damage = lhd_left
                                current.least_hp_hp = lhh_left
                            else:
                                current.least_hp_damage = lhd_above
                                current.least_hp_hp = lhh_above
                        # ldh_above <= lhh_above
                        else:
                            if lhh_left == ldh_above:
                                if lhd_left < ldd_above:
                                    current.least_hp_damage = lhd_left
                                    current.least_hp_hp = lhh_left
                                else:
                                    current.least_hp_damage = ldd_above
                                    current.least_hp_hp = ldh_above
                            elif lhh_left < ldh_above:
                                current.least_hp_damage = lhd_left
                                current.least_hp_hp = lhh_left
                            else:
                                current.least_hp_damage = ldd_above
                                current.least_hp_hp = ldh_above
                    
                    # ldh_left <= lhh_left - in case of a tie, less damage wins
                    else:
                        # then determine which route from above takes less hp
                        if lhh_above < ldh_above:
                            # then determine whether left or above takes less hp
                            # in case of a tie, sort by damage
                            if ldh_left == lhh_above:
                                if ldd_left < lhd_above:
                                    current.least_hp_damage = ldd_left
                                    current.least_hp_hp = ldh_left
                                else:
                                    current.least_hp_damage = lhd_above
                                    current.least_hp_hp = lhh_above
                            elif ldh_left < lhh_above:
                                current.least_hp_damage = ldd_left
                                current.least_hp_hp = ldh_left
                            else:
                                current.least_hp_damage = lhd_above
                                current.least_hp_hp = lhh_above
                        # ldh_above <= lhh_above
                        else:
                            if ldh_left == ldh_above:
                                if ldd_left < ldd_above:
                                    current.least_hp_damage = ldd_left
                                    current.least_hp_hp = ldh_left
                                else:
                                    current.least_hp_damage = ldd_above
                                    current.least_hp_hp = ldh_above
                            elif ldh_left < ldh_above:
                                current.least_hp_damage = ldd_left
                                current.least_hp_hp = ldh_left
                            else:
                                current.least_hp_damage = ldd_above
                                current.least_hp_hp = ldh_above

        return dp[(row_count - 1) % 2][col_count - 1].least_hp_hp