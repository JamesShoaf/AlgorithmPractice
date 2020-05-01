/*
You are a product manager and currently leading a team to develop a new product. Unfortunately, the
latest version of your product fails the quality check. Since each version is developed based on the
previous version, all the versions after a bad version are also bad.

Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes
all the following ones to be bad.

You are given an API bool isBadVersion(version) which will return whether version is bad. Implement
a function to find the first bad version. You should minimize the number of calls to the API.
*/

const isBadVersionGenerator = (badVersionNumber) => (
  (versionNumber) => Boolean(versionNumber >= badVersionNumber)
);

const firstBadVersion = (isBadVersion) => (
  (versionCount) => {
    if (isBadVersion(1)) {
      return 1;
    }
    let lastKnownGood = 1;
    let firstKnownBad = versionCount;
    while (firstKnownBad - lastKnownGood > 1) {
      const midpoint = Math.floor((firstKnownBad - lastKnownGood) / 2 + lastKnownGood);
      if (isBadVersion(midpoint)) {
        firstKnownBad = midpoint;
      } else {
        lastKnownGood = midpoint;
      }
    }
    return firstKnownBad;
  }
);

module.exports = {
  isBadVersionGenerator,
  firstBadVersion,
};
