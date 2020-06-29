# Given a list of airline tickets represented by pairs of departure and arrival
# airports [from, to], reconstruct the itinerary in order. All of the tickets
# belong to a person who departs from JFK. Thus, the itinerary must begin with JFK.

from typing import List

class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        # iterate through the tickets and create a sorted list of all cities
        cities = set()
        for origin, destination in tickets:
            cities.add(origin)
            cities.add(destination)
        cities = sorted(cities)
        city_indexes = {}
        for index, city in enumerate(cities):
            city_indexes[city] = index
        out_edges_from_index = [{'out_count': 0, 'sorted_edges': []} for i in range(len(cities))]

        for origin, destination in tickets:
            index = city_indexes[origin]
            destinations = out_edges_from_index[index]
            if not destinations.get(destination): destinations[destination] = 0
            destinations[destination] += 1
            destinations['out_count'] += 1

        # in descending order, append destination cities to each origin city's sorted_edges
        # thus, the out_count - 1 points to the first unvisited edge alphabetically
        for origin in out_edges_from_index:
            for destination in reversed(cities):
                if origin.get(destination):
                    for edge in range(origin[destination]):
                        origin['sorted_edges'].append(destination)
        path = []

        # while there are edges to traverse, recurse on the next edge
        # when there are none left, push the current node to the path
        # then backtrack to the previous call
        def dfs(city: str) -> None:
            origin = out_edges_from_index[city_indexes[city]]
            while(origin['out_count']):
                origin['out_count'] -= 1
                next_city = origin['sorted_edges'][origin['out_count']]
                dfs(next_city)
            path.append(city)
        
        dfs('JFK')
        # when all paths within the connected component have been traversed, return the array
        return reversed(path)