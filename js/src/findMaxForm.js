export default function findMaxForm(strs, m, n) {
  let count = 0;
  strs.forEach((str) => {
    let array = [...str];
    let ones = array.filter((s) => s === "1").length;
    if (n < ones) {
      count += ones;
    }
  });
  return count;
}

console.log(findMaxForm(["10", "0001", "111001", "1", "0"], 5, 3));
console.log(findMaxForm(["10", "0", "1"], 1, 1));
