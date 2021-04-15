/**
 * https://www.youtube.com/watch?v=XnMI5jIfJNc
 * https://app.codesignal.com/arcade/intro/level-2/2mxbGwLzvkTCKAJMG/solutions
 * @param {*} sequence
 * @returns
 */
export default function almostIncreasingSequence(sequence) {
  let target = 0,
    max = Math.pow(-10, 5),
    secondMax = Math.pow(-10, 5);
  sequence.forEach((elem) => {
    if (elem > max) {
      secondMax = max;
      max = elem;
    } else if (elem > secondMax) {
      max = elem;
      target++;
    } else {
      target++;
    }
  });
  return target <= 1;
}

console.log(almostIncreasingSequence([1, 3, 2, 1]) === false);
console.log(almostIncreasingSequence([1, 3, 2]) === true);
console.log(almostIncreasingSequence([1, 2, 1, 2]) === false);
console.log(almostIncreasingSequence([3, 6, 5, 8, 10, 20, 15]) === false);
console.log(almostIncreasingSequence([1, 1, 2, 3, 4, 4]) === false);
console.log(almostIncreasingSequence([1, 4, 10, 4, 2]) === false);
console.log(almostIncreasingSequence([10, 1, 2, 3, 4, 5]) === false);
console.log(almostIncreasingSequence([1, 1, 1, 2, 3]) === false);
console.log(almostIncreasingSequence([0, -2, 5, 6]) === true);
console.log(almostIncreasingSequence([1, 2, 3, 4, 5, 3, 5, 6]) === false);
console.log(almostIncreasingSequence([40, 50, 60, 10, 20, 30]) === false);
console.log(almostIncreasingSequence([1, 2]) === true);
console.log(almostIncreasingSequence([1, 2, 5, 3, 5]) === true);
console.log(almostIncreasingSequence([1, 2, 5, 5, 5]) === false);
console.log(almostIncreasingSequence([10, 1, 2, 3, 4, 5, 6, 1]) === false);
console.log(almostIncreasingSequence([1, 2, 3, 4, 3, 6]) === true);
console.log(almostIncreasingSequence([1, 2, 3, 4, 99, 5, 6]) === true);
console.log(
  almostIncreasingSequence([123, -17, -5, 1, 2, 3, 12, 43, 45]) === true
);
console.log(almostIncreasingSequence([3, 5, 67, 98, 3]) === true);
