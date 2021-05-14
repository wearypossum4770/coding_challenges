export default function countTinyPairs(a, b, k) {
  let target = Array.from(a.length);
  let value;
  a = a.map((val) => val.toString());
  b = b.reverse().map((val) => val.toString());
  for (let i = 0; i < a.length; i++) {
    value = parseInt(a[i] + b[i]);
    console.log(value < k);
    if (value < k) {
      target[i] = value;
      value = 0;
    }
  }
  return target.filter((val) => val).length;
}
console.log(countTinyPairs([16, 1, 4, 2, 14], [7, 11, 2, 0, 15], 743));
console.log(countTinyPairs([1, 2, 3], [1, 2, 3], 31));
