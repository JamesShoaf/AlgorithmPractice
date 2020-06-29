import unittest
from reconstruct_itinerary import Solution

class TestReconstructItinerary(unittest.TestCase):
    solver = Solution()


    def test_invalid_itinerary(self):
        test_tuples = [
            ([["JFK", "SFO"], ["JFK", "ATL"]],
            ["JFK","SFO","ATL"]),
            ([["JFK", "SFO"], ["LHR", "ATL"]],
            ["JFK","SFO"]),
        ]
        for tickets, expected in test_tuples:
            actual = self.solver.findItinerary(tickets)
            for index, location in enumerate(actual):
                self.assertEqual(location, expected[index])


    def test_nonlooping_itinerary(self):
        test_tuples = [
            ([["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]],
            ["JFK","MUC","LHR","SFO","SJC"]),
        ]
        for tickets, expected in test_tuples:
            actual = self.solver.findItinerary(tickets)
            for index, location in enumerate(actual):
                self.assertEqual(location, expected[index])


    def test_looping_itinerary(self):
        test_tuples = [
            ([["JFK", "SFO"], ["JFK", "SFO"], ["SFO", "ATL"], ["ATL", "JFK"]],
            ["JFK", "SFO", "ATL", "JFK", "SFO"])
        ]
        for tickets, expected in test_tuples:
            actual = self.solver.findItinerary(tickets)
            for index, location in enumerate(actual):
                self.assertEqual(location, expected[index])

if __name__ == '__main__':
    unittest.main()