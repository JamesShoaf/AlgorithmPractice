/*
There are n cities numbered from 0 to n-1 and n-1 roads such that there is only one way to travel
between two different cities (this network form a tree). Last year, The ministry of transport
decided to orient the roads in one direction because they are too narrow.

Roads are represented by connections where connections[i] = [a, b] represents a road from city a
to b.

This year, there will be a big event in the capital (city 0), and many people want to travel to this
city.

Your task consists of reorienting some roads such that each city can visit the city 0. Return the
minimum number of edges changed.

It's guaranteed that each city can reach the city 0 after reorder.
*/

const minReorder = (n, connections) => {
  const cities = [...Array(n)].map(() => []);
  connections.forEach((route) => {
    const [origin, destination] = route;
    cities[origin].push(route);
    cities[destination].push(route);
  });
  const visited = new Set();
  // start at city 0
  const queue = [0];
  let output = 0;
  while (queue.length) {
    const currentCity = queue.pop();
    // mark the current city as visited
    visited.add(currentCity);
    const currentRoutes = cities[currentCity];
    // for each route, add the unvisited end to the queue
    const { length } = currentRoutes;
    for (let i = 0; i < length; i += 1) {
      const [origin, destination] = currentRoutes[i];
      // if the destination is unvisited, this route will need to be reversed.
      if (!visited.has(destination)) {
        queue.push(destination);
        output += 1;
      }
      if (!visited.has(origin)) {
        queue.push(origin);
      }
    }
  }
  return output;
};

module.exports = { minReorder };
