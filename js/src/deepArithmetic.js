export default function deepArithmetic(array) {
  let total = 0;
  let regex = new RegExp(/[0-9]|\-[0-9]/);
  let nums = array.flat(Infinity);

  for (let outer = 0; outer < nums.length; outer++) {
    let num = nums[outer].match(regex);
    if (num) {
      for (let inner = 0; inner < nums[outer].length; inner++) {
        console.log(nums[inner]);
      }
    }
  }
  return nums;
  return total;
}

console.log(deepArithmetic(["1", "five", "2wenty-one", "thr33"]));
// console.log(deepArithmetic([["1X2", "t3n"],["1024", "5", "64"]],))
// console.log(deepArithmetic([[["1"], "10v3"], ["738h"], [["s0"], ["1mu4ch3"],"-1s0"]],))
// console.log(deepArithmetic([[["0", "0x2", "z3r1"],["1", "55a46"]],[["1", "0b2", "4"],["0x5fp-2", "nine", "09"],["4", "4", "4"]],[["03"]], []],))
// console.log(deepArithmetic([[[[[[[[[[[[[[[["-1", "1"], ["3"], [""], []]]]]]]]]]]]]]]],))
// console.log(deepArithmetic([[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]],))
console.log(deepArithmetic([[[[[["-32-64", "a-zA-Z"], ["01-1"]]]]]]));
