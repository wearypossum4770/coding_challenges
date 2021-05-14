export default function getDeterminant(matrix) {
  // 1x1 [a] = a
  // 2x2 [[a,b][c,d]] = ad-bc
  //  3x3 [[a,b,c,],[d,e,f],[g,h,i]] = a[[]]
  let [outer, inner] = [matrix.length, matrix[0].length];

  if (outer === inner) {
    const loop = () => {
      for (let i = 0; i < matrix.length; i++) {
        let coefficient = matrix;
      }
    };
    switch (outer) {
      case 1:
        return matrix;
      case 2:
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }
  }
}

console.log(
  getDeterminant([
    [1, 2],
    [-1, 3],
  ]) === 5
);
console.log(
  getDeterminant([
    [0, 1],
    [1, 1],
  ]) === -1
);
console.log(
  getDeterminant([
    [69, 0],
    [1, 1],
  ]) === 69
);
