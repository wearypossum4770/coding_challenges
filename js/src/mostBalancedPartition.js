function mostBalancedPartition(parent, files_size) {
  let lPointer = (rPointer = []);
  let target = files_size.reduce((accum, currVal) => accum + currVal, 0) / 2;
  for (let index = 0; index < files_size.length; index++) {}
}

console.log(mostBalancedPartition([-1, 0, 0, 1, 1, 2], [1, 2, 2, 1, 1, 1]));
console.log(mostBalancedPartition([-1, 0, 1, 2], [-1, 0, 1, 2]));
