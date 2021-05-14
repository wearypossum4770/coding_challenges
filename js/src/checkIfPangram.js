/**
 * https://leetcode.com/problems/check-if-the-sentence-is-pangram/submissions/
 * Runtime: 108 ms, faster than 17.35% of JavaScript online submissions for Check if the Sentence Is Pangram.
 * Memory Usage: 38.3 MB, less than 91.78% of JavaScript online submissions for Check if the Sentence Is Pangram.
 * @param {string} sentence
 * @return {boolean}
 */
var checkIfPangram = function (sentence) {
  let is_pangram = true;
  let letters = [..."abcdefghijklmnopqrstuvwxyz"];
  letters.forEach((letter) => {
    if (!sentence.includes(letter)) {
      is_pangram = false;
    }
  });
  return is_pangram;
};
