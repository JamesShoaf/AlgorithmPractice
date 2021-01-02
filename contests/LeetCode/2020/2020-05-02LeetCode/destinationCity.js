/*
You are given the array paths, where paths[i] = [cityAi, cityBi] means there exists a
direct path going from cityAi to cityBi. Return the destination city, that is, the city
without any path outgoing to another city.

It is guaranteed that the graph of paths forms a line without any loop, therefore, there
will be exactly one destination city.
*/

const destinationCity = (paths) => {
  const map = {};
  paths.forEach((path) => {
    if (map[path[0]] === undefined) {
      map[path[0]] = [path[0]];
    }
  });
  for (let i = 0; i < paths.length; i += 1) {
    if (map[paths[i][1]] === undefined) {
      return paths[i][1];
    }
  }
  return null;
};

module.exports = {
  destinationCity,
};
