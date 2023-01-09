const binary = [1, 2, 4, 8, 16, 32, 64, 128];
const addBinary = (a, b) => {
  let left = 0;
  let right = 0;
  let target = ''
  let max = Math.max.call(null, a.length, b.length) - 1;
  for (let i = max; i >= 0; i--) {
    if (a[i]) {
      left += parseInt(a[i]) ? binary[max - i] : 0;
    }
    if (b[i]) {
      right += parseInt(b[i]) ? binary[max - i] : 0;
    }
  }
  let sum = left + right;
  while(sum) {
    for (let i =max; i>=0; i--) {
      target + (Math.floor(sum/binary[i])) ? 1:0
      sum -= binary[i]
    }
  }
  return targetçç;
};
// console.log(addBinary("11", "1"))
console.log(addBinary("1010", "1011"));
// export default addBinary
// 10101
