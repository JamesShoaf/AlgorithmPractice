class RadInt {
  constructor(value = '0', radix = 10) {
    this.value = (typeof value === 'string') ? value : Number(value).toString(radix);
    this.radix = radix;
  }

  plus(radInt) {
    const { radix, value } = this;
    if (radix !== radInt.radix) {
      return null;
    }
    const value2 = radInt.value;
    let sum = '';
    let carryBit = 0;
    let lastIndex = value.length - 1;
    let lastIndex2 = value2.length - 1;
    while (lastIndex >= 0 || lastIndex2 >= 0) {
      let runningSum = carryBit;
      carryBit = 0;
      const parse = Number.parseInt(value[lastIndex], radix);
      if (!Number.isNaN(parse)) {
        runningSum += parse;
      }
      const parse2 = Number.parseInt(value2[lastIndex2], radix);
      if (!Number.isNaN(parse2)) {
        runningSum += parse2;
      }
      if (runningSum >= radix) {
        runningSum -= radix;
        carryBit += 1;
      }
      sum = `${runningSum}${sum}`;
      lastIndex -= 1;
      lastIndex2 -= 1;
    }
    if (carryBit) {
      sum = `${carryBit}${sum}`;
    }
    return new RadInt(sum, radix);
  }

  minus(radInt) {
    const { radix, value } = this;
    if (radix !== radInt.radix) {
      return null;
    }
    let difference = '';
    let carryBit = 0;
    const value2 = radInt.value;
    let lastIndex = value.length - 1;
    let lastIndex2 = value2.length - 1;
    while (lastIndex >= 0 || lastIndex2 >= 0) {
      let runningDiff = carryBit;
      carryBit = 0;
      const parse = Number.parseInt(value[lastIndex], radix);
      if (!Number.isNaN(parse)) {
        runningDiff += parse;
      }
      const parse2 = Number.parseInt(value2[lastIndex2], radix);
      if (!Number.isNaN(parse2)) {
        runningDiff -= parse2;
      }
      if (runningDiff < 0) {
        runningDiff += radix;
        carryBit -= 1;
      }
      if (runningDiff !== 0 || lastIndex > 0) {
        difference = `${runningDiff}${difference}`;
      }
      lastIndex -= 1;
      lastIndex2 -= 1;
    }
    return new RadInt(difference, radix);
  }
}

module.exports = {
  RadInt,
};
