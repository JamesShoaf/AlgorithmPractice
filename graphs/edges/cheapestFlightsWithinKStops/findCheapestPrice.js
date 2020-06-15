const { PriorityQueue: PQ } = require('../../../heaps/priorityQueue');

/*
There are n cities connected by m flights. Each flight starts from city u and arrives
at v with a price w.

Now given all the cities and flights, together with starting city src and the destination
dst, your task is to find the cheapest price from src to dst with up to k stops. If there
is no such route, output -1.


    The number of nodes n will be in range [1, 100], with nodes labeled from 0 to n - 1.
    The size of flights will be in range [0, n * (n - 1) / 2].
    The format of each flight will be (src, dst, price).
    The price of each flight will be in the range [1, 10000].
    k is in the range of [0, n - 1].
    There will not be any duplicated flights or self cycles.

*/

// use Dijkstra's algorithm to find the cheapest path from the source node to each other node

const findCheapestPrice = (n, flights, src, dst, K) => {
  // convert edges to a map of prices to travel between nodes
  const nodes = [...Array(n)].map(() => []);
  const { length: numFlights } = flights;
  for (let i = 0; i < numFlights; i += 1) {
    const [source, destination, price] = flights[i];
    nodes[source].push([destination, price]);
  }

  // min heap sorted by price
  const queue = new PQ((a, b) => a[1] < b[1]);
  // map of shortest routes from one node to another
  // shortestRoutes[source][destination]
  const shortestRoutes = [...Array(n)].map(() => ({}));
  // [destination, totalPrice, totalStops]
  queue.push([src, 0, -1]);
  // travel to the least expensive node from the starting node
  while (!queue.isEmpty()) {
    const [source, price, stops] = queue.pop();
    // when a valid route to the destination has been found, return the price
    if (source === dst) return price;
    // prevent long routes by only adding destinations within K nodes
    if (stops < K) {
      const destinations = nodes[source];
      destinations.forEach(([destination, destPrice]) => {
        const totalPrice = price + destPrice;
        const totalStops = stops + 1;
        // only add previously travelled routes to the queue if the route is the same length
        // or shorter
        if (!(totalStops > shortestRoutes[source][destination])) {
          shortestRoutes[source][destination] = totalStops;
          queue.push([destination, totalPrice, totalStops]);
        }
      });
    }
  }
  // if no valid routes are found, return -1
  return -1;
};

module.exports = { findCheapestPrice };
