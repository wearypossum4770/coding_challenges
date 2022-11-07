/**
 * Forward declaration of guess API.
 * @param {number} num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * const guess = function(num) {}
 */

/**
 * @param {number} n
 * @return {number}
 */
const randNum = 6 //?? Math.floor(Math.random() * (Math.pow(2, 31) - 1));
const guess = (n) => {
  if (n === randNum) return 0;
  if (n > randNum) return 1;
  return -1;
};
const guessNumber = (n) => {
  let high = n;
  let low = 1;
  while (low <= high) {
    const mid = low + (high - low) / 2;
    const res = guess(n);
    if (res === 0) return res;
    else if (res < 0) {
      high = mid - 1;
    } else {
      low = mid + 1;
    }
  }
  return -1;
};

console.log(guessNumber(10));
