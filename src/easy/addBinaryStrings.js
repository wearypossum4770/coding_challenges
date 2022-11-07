const binary = [128, 64, 32, 16, 8, 4, 2, 1];
const addBinaryStrings = (a, b) => {
  if (a === "0" && b === "0") return "0";
  let left = Array.from("0".repeat(8).concat(a).slice(-binary.length)).reduce(
    (accum, curr, index) => (accum += parseInt(curr) ? binary[index] : 0),
    0
  );
  let right = Array.from("0".repeat(8).concat(b).slice(-binary.length)).reduce(
    (accum, curr, index) => (accum += parseInt(curr) ? binary[index] : 0),
    0
  );
  let num = left + right;
  console.log(left, right);
  let target = "";
  for (let i = 0; i < binary.length; i++) {
    if (num % binary[i] < num) {
      target += 1;
      num -= binary[i];
    } else {
      target += 0;
    }
  }
  return target.replace(/^0*/, "");
};

// [["11", "1"], "100"],
// [["1010", "1011"], "1010"],
// [["0", "0"], "0"],
console.log(addBinaryStrings("0", "1"));
console.log(addBinaryStrings("1110001", "110100100") === "1000010101");

// // export default addBinaryStrings
// console.log( 128, 21 % 128 , 21 % 128 < 21)
// console.log( 64, 21 % 64 , 21 % 64 < 21)
// console.log( 32, 21 % 32 , 21 % 32 < 21)
// console.log( 16, 21 % 16 , 21 % 16 < 21)
// console.log( 8, 21 % 8 , 21 % 8 < 21)
// console.log( 4, 21 % 4 , 21 % 4 < 21)
// console.log( 2, 21 % 2 , 21 % 2 < 21)
// console.log( 1, 21 % 1 , 21 % 1 < 21)
