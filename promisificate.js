// converts normal functions into promisified functions which return promises that resolve
// after 1 second

const promisificate = (func) => (...params) => new Promise((resolve, reject) => {
  setTimeout(() => (func.call(null, ...params)
    ? resolve(func.call(null, ...params))
    : reject(new Error('falsey value'))),
  1000);
});

module.exports = promisificate;
