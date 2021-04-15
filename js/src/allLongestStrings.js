/**
 * @function allLongestStrings captures strings with longest lengths
 * @param {Array.<string>} inputArray
 * @returns {Array.<string>} array of strins with longest values
 * @module allLongestStrings/allLongestStrings
 * @exports allLongestStrings
 * @copyright retrieved from: https://app.codesignal.com/arcade/intro/level-3/fzsCQGYbxaEcTr2bL
 *
 */
export default function allLongestStrings(inputArray) {
  let max = Math.max(...inputArray.map((str) => str.length));
  return inputArray.filter((str) => str.length === max);
}

console.log(allLongestStrings(["aba", "aa", "ad", "vcd", "aba"]));
