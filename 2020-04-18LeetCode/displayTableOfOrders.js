const tableDisplay = (orders) => {
  const entrees = {};
  const tables = {};
  orders.forEach((order) => {
    entrees[order[2]] = true;
    if (!tables[order[1]]) {
      tables[order[1]] = {};
      tables[order[1]][order[2]] = 1;
    } else if (!tables[order[1]][order[2]]) {
      tables[order[1]][order[2]] = 1;
    } else {
      tables[order[1]][order[2]] += 1;
    }
  });

  const entreeList = Object.keys(entrees).sort();
  const headerArray = ['Table', ...entreeList];
  const tableList = Object.keys(tables).map(
    (number) => Number.parseInt(number, 10),
  ).sort((a, b) => a - b);

  const output = [headerArray];
  tableList.forEach((tableNumber) => {
    const tableArray = [`${tableNumber}`];
    entreeList.forEach((entree) => {
      if (!tables[tableNumber][entree]) {
        tableArray.push('0');
      } else {
        tableArray.push(`${tables[tableNumber][entree]}`);
      }
    });
    output.push(tableArray);
  });
  return output;
};

const orders = [
  ['David', '3', 'Ceviche'],
  ['Corina', '10', 'Beef Burrito'],
  ['David', '3', 'Fried Chicken'],
  ['Carla', '5', 'Water'],
  ['Carla', '5', 'Ceviche'],
  ['Rous', '3', 'Ceviche'],
];

tableDisplay(orders);
