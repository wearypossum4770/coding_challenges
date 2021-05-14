export default function fibonacci(n) {
  let root = Math.sqrt(5);
  return Math.round(
    (Math.pow(1 + root, n) - Math.pow(1 - root, n)) / (Math.pow(2, n) * root)
  );
}

console.log(fibonacci(0) === 0);
console.log(fibonacci(1) === 1);
console.log(fibonacci(2) === 1);
console.log(fibonacci(3) === 2);
console.log(fibonacci(4) === 3);
console.log(fibonacci(5) === 5);
console.log(fibonacci(6) === 8);
console.log(fibonacci(7) === 13);
console.log(fibonacci(8) === 21);
console.log(fibonacci(9) === 34);
console.log(fibonacci(10) === 55);
console.log(fibonacci(11) === 89);
console.log(fibonacci(12) === 144);
console.log(fibonacci(13) === 233);
console.log(fibonacci(14) === 377);
console.log(fibonacci(15) === 610);
console.log(fibonacci(16) === 987);
console.log(fibonacci(17) === 1597);
console.log(fibonacci(18) === 2584);
console.log(fibonacci(19) === 4181);
console.log(fibonacci(20) === 6765);
console.log(fibonacci(21) === 10946);
console.log(fibonacci(22) === 17711);
console.log(fibonacci(23) === 28657);
console.log(fibonacci(24) === 46368);
console.log(fibonacci(25) === 75025);
console.log(fibonacci(26) === 121393);
console.log(fibonacci(27) === 196418);
console.log(fibonacci(28) === 317811);
console.log(fibonacci(29) === 514229);
console.log(fibonacci(30) === 832040);
