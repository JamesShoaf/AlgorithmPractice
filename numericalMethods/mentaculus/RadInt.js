class RadInt {
  constructor(value = '0', radix = 10) {
    if (!((typeof value === 'string' && value.search(/\D/) === -1 && (value[0] !== '0' || value === '0') && value !== '')
        || (typeof value === 'number' && Number.isInteger(value) && value >= 0))
      || typeof radix !== 'number' || !(radix > 1) || !Number.isInteger(radix)) {
      this.value = '0';
      this.radix = 10;
      return;
    }
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
      difference = `${runningDiff}${difference}`;
      lastIndex -= 1;
      lastIndex2 -= 1;
    }
    for (let i = 0; i < difference.length; i += 1) {
      if (difference[i] !== '0') {
        return new RadInt(difference.slice(i), radix);
      }
    }
    return new RadInt(difference, radix);
  }

  times(radInt) {
    const { radix, value } = this;
    if (radix !== radInt.radix) {
      return null;
    }
    const value2 = radInt.value;
    let product = new RadInt(0, radix);
    let runningSum;
    let carryBit;
    const lastIndex = value.length - 1;
    const lastIndex2 = value2.length - 1;
    for (let i = lastIndex; i >= 0; i -= 1) {
      runningSum = '';
      carryBit = 0;
      const parse = Number.parseInt(value[i], radix);
      for (let k = 0; k < lastIndex - i; k += 1) {
        runningSum += '0';
      }
      for (let j = lastIndex2; j >= 0; j -= 1) {
        const parse2 = Number.parseInt(value2[j], radix);
        let subProduct = parse * parse2;
        subProduct += carryBit;
        runningSum = `${subProduct % radix}${runningSum}`;
        carryBit = Math.floor(subProduct / radix);
      }
      if (carryBit > 0) {
        runningSum = `${carryBit}${runningSum}`;
      }
      if (/* runningSum.search(/[^0]/) !== -1 */ runningSum[0] !== '0') {
        product = product.plus(new RadInt(runningSum, radix));
      }
    }
    return product;
  }
}

module.exports = {
  RadInt,
};
