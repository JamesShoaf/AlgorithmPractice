// converts normal functions into promisified functions which return promises that resolve
// after 1 second

const promisificate = (func) => (...params) => new Promise((resolve, reject) => {
  setTimeout(() => (func.call(null, ...params)
    ? resolve(func.call(null, ...params))
    : reject(new Error('falsey value'))),
  1000);
});

const valPlusCallback = (cb) => {
  const val = Math.floor(2 * Math.random());
  setTimeout(cb, 1000, val);
  return Boolean(Math.floor(2 * Math.random()));
};

const promiseValBack = () => {
  const output = [];
  output.push(new Promise((resolve) => {
    output.push(valPlusCallback(resolve));
  }));
  return output;
};

module.exports = promisificate;
