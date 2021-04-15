/**
 * https://app.codesignal.com/arcade/intro/level-2/xskq4ZxLyqQMCLshr/solutions
 * @param {*} matrix
 * @returns
 */
export default function matrixElementsSum(matrix) {
  let depth = matrix.length,
    rounds = matrix[0].length,
    target = [],
    array = [],
    value = 0;
  while (rounds > 0) {
    matrix.forEach((element) => array.push(element.pop()));
    rounds--;
  }
  while (array.length) {
    target.push(array.splice(0, depth));
  }
  target.forEach((val) => {
    for (let i = 0; i < val.length; i++) {
      if (val[i] === 0) {
        break;
      } else {
        value += val[i];
      }
    }
  });
  return value;
}
console.log(
  matrixElementsSum([
    [0, 1, 1, 2],
    [0, 5, 0, 0],
    [2, 0, 3, 3],
  ])
);
