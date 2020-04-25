/*
There exists a property of place value such that multiplying an ascending set
of integers by the radix minus 2 and adding a small remainder results in a descending set
of integers of the same length.

In base 10:
            1 * 8 + 1 = 9
           12 * 8 + 2 = 98
          123 * 8 + 3 = 987
         1245 * 8 + 4 = 9876
        12345 * 8 + 5 = 98765
       123456 * 8 + 6 = 987654
      1234567 * 8 + 7 = 9876543
     12345678 * 8 + 8 = 98765432
    123456789 * 8 + 9 = 987654321


There are two potential ways to continue this pattern
123...8912...  -  987...2198...
OR
123...8901...  -  987...2109...

which result in different remainders
91, 902, 9003, 90004...
90, 901, 9002, ...
*/

const mentaculus = {
  generateSkipAscending: (length, radix = 10) => {
    let output = '';
    for (let i = 0; i < length; i += 1) {
      const mod = (i + 1) % (radix - 1);
      const nextInt = (mod === 0) ? radix - 1 : mod;
      output += `${nextInt}`;
    }
    return output;
  },
  generateAscending: (length, radix = 10) => {
    let output = '';
    for (let i = 0; i < length; i += 1) {
      const nextInt = (i + 1) % radix;
      output += `${nextInt}`;
    }
    return output;
  },
  generateSkipDescending: (length, radix = 10) => {
    let output = '';
    for (let i = 0; i < length; i += 1) {
      // 9 * (1 + floor(i/9)) + 1 - i
      const nextInt = (radix - 1) * (1 + Math.floor(i / (radix - 1))) - i;
      output += `${nextInt}`;
    }
    return output;
  },
  generateDescending: (length, radix = 10) => {
    let output = '';
    for (let i = 0; i < length; i += 1) {
      const mod = radix - ((i + 1) % radix);
      const nextInt = (mod === radix) ? 0 : mod;
      output += `${nextInt}`;
    }
    return output;
  },
  skipRemainder: (length, radix = 10) => {
    const ascending = BigInt(mentaculus.generateSkipAscending(length, radix));
    const descending = BigInt(mentaculus.generateSkipDescending(length, radix));
    const remainder = descending - BigInt(radix - 2) * ascending;
    return String(remainder);
  },
  remainder: (length, radix = 10) => {
    const ascending = BigInt(mentaculus.generateAscending(length, radix));
    const descending = BigInt(mentaculus.generateDescending(length, radix));
    const remainder = descending - BigInt(radix - 2) * ascending;
    return String(remainder);
  },
};

// for (let i = 1; i < 31; i += 1) {
//   const string = `${mentaculus.generateSkipDescending(i)}
//    - 8 * ${mentaculus.generateSkipAscending(i)}
//    = ${mentaculus.skipRemainder(i)}`;
//   console.log(string);
// }

module.exports = mentaculus;
