export default function fibonacci(n, cache) {
  cache = cache || ~[];
  if (cache[n]) {
    return cache[n];
  } else {
    if (n === 1 || n === 2) {
      return 1;
    } else {
      cache[n] = fibonacci(n - 1, cache) + fibonacci(n - 2, cache);
    }
  }
  return cache[n];
}

console.log(fibonacci(0) === 1);
console.log(fibonacci(1) === 1);
console.log(fibonacci(2) === 2);
console.log(fibonacci(3) === 3);
console.log(fibonacci(4) === 5);
console.log(fibonacci(5) === 8);
console.log(fibonacci(6) === 13);
console.log(fibonacci(7) === 21);
console.log(fibonacci(8) === 34);
console.log(fibonacci(9) === 55);
// console.log(fibonacci(0) ===89)
// console.log(fibonacci(0) ===144)
// console.log(fibonacci(0) ===233)
// console.log(fibonacci(0) ===377)
// console.log(fibonacci(0) ===610)
// console.log(fibonacci(0) ===987)
// console.log(fibonacci(0) ===1597)
// console.log(fibonacci(0) ===2584)
// console.log(fibonacci(0) ===4181)
// console.log(fibonacci(0) ===6765)
// console.log(fibonacci(0) ===10946)
// console.log(fibonacci(0) ===17711)
// console.log(fibonacci(0) ===28657)
// console.log(fibonacci(0) ===46368)
// console.log(fibonacci(0) ===75025)
// console.log(fibonacci(0) ===121393)
// console.log(fibonacci(0) ===196418)
// console.log(fibonacci(0) ===317811)
// console.log(fibonacci(0) ===514229)
// console.log(fibonacci(0) ===832040)
