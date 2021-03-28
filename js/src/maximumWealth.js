export default function maximumWealth(accounts = []) {
  let maximum = accounts.map((array) =>
    array.reduce((accum, currVal) => accum + currVal, 0)
  );
  return Math.max(...maximum);
}

console.log(
  maximumWealth([
    [2, 8, 7],
    [7, 1, 3],
    [1, 9, 5],
  ])
);
// console.log(maximumWealth([[2,8,7],[7,1,3],[1,9,5]]) === 10)
