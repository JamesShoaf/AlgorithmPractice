// split string into letters and numbers

const reformatString = (string) => {
  const chars = string.split('');
  const digits = chars.filter((char) => Number.isInteger(Number.parseInt(char, 10)));
  const letters = chars.filter((char) => !Number.isInteger(Number.parseInt(char, 10)));

  const digitCount = digits.length;
  const letterCount = letters.length;

  if (Math.abs(digitCount - letterCount) > 1) {
    return '';
  }

  let newString = '';

  if (digitCount > letterCount) {
    for (let i = 0; i < string.length; i += 1) {
      const currentIndex = Math.floor(i / 2);
      if (i % 2 === 0) {
        newString += digits[currentIndex];
      } else {
        newString += letters[currentIndex];
      }
    }
  } else {
    for (let i = 0; i < string.length; i += 1) {
      const currentIndex = Math.floor(i / 2);
      if (i % 2 === 0) {
        newString += letters[currentIndex];
      } else {
        newString += digits[currentIndex];
      }
    }
  }

  return newString;
};

reformatString('take');
