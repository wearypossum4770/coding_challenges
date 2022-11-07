/**
 *
 * @param {number} rowIndex - 0th-indexed row of the pascal triangle
 * @return {Array.<number>}
 */
const pascalsTriangle2 = (rowIndex) => {
  let row = [];
  const target = Array(33);
  for (let i = 0; i < target.length; i++) {
    target[i] = Array(i + 1);
    for (let j = 0; j < target[i].length; j++) {
      if (j === rowIndex) return target[rowIndex].splice(0);
      if (j === 0 || j === target[i].length - 1) {
        target[i][j] = 1;
      } else {
        target[i][j] = target[i - 1][j] + target[i - 1][j - 1];
      }
    }
  }
  return row;
};

// export default pascalsTriangle2

console.log(pascalsTriangle2(32));
