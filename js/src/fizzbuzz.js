function fizzBuzz(n) {
  for (let num = 1; num < n + 1; num++) {
    if (num % 3 === 0 && num % 5 === 0) {
      console.log("FizzBuzz");
    }
    if (num % 3 === 0 && num % 5 !== 0) {
      console.log("Fizz");
    }
    if (num % 3 !== 0 && num % 5 === 0) {
      console.log("Buzz");
    }
    if (num % 3 !== 0 && num % 5 !== 0) {
      console.log(num);
    }
  }

  // Write your code here
}

console.log(fizzBuzz(15));
const test_cases = [15, 65, 12, 688, 225, 3626, 84098, 197145];
function isMultiple(num, mod){
  return num%mod ===0
}
function FizzBuzz(num){
  switch(true){
    case isMultiple(num, 15):
      return "FizzBuzz"
    case isMultiple(num,3):
      return "Fizz"
    case isMultiple(num, 5):
      return "Buzz"
    default:
      return num;
  }
}
const main = [...Array(100)].map((_,i)=>FizzBuzz(i))