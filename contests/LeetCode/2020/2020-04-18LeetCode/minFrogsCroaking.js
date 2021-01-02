const minNumberOfFrogs = (croakOfFrogs) => {
  const croakCounts = {
    c: 0,
    cr: 0,
    cro: 0,
    croa: 0,
  };
  let minCroaks = 0;
  const croakLength = croakOfFrogs.length;
  const croakCheck = () => Object.values(croakCounts).reduce((acc, val) => acc + val, 0);
  for (let i = 0; i < croakLength; i += 1) {
    const letter = croakOfFrogs[i];
    if (letter === 'c') {
      croakCounts.c += 1;
      minCroaks = Math.max(minCroaks, croakCheck());
    } else if (letter === 'r') {
      if (croakCounts.c === 0) {
        return -1;
      }
      croakCounts.c -= 1;
      croakCounts.cr += 1;
      minCroaks = Math.max(minCroaks, croakCheck());
    } else if (letter === 'o') {
      if (croakCounts.cr === 0) {
        return -1;
      }
      croakCounts.cr -= 1;
      croakCounts.cro += 1;
      minCroaks = Math.max(minCroaks, croakCheck());
    } else if (letter === 'a') {
      if (croakCounts.cro === 0) {
        return -1;
      }
      croakCounts.cro -= 1;
      croakCounts.croa += 1;
      minCroaks = Math.max(minCroaks, croakCheck());
    } else if (letter === 'k') {
      if (croakCounts.croa === 0) {
        return -1;
      }
      croakCounts.croa -= 1;
      minCroaks = Math.max(minCroaks, croakCheck());
    } else {
      return -1;
    }
  }
  return croakCheck() ? -1 : minCroaks;
};

minNumberOfFrogs('croakcrook');
