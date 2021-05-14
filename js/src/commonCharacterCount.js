/**
 *  @copyright https://app.codesignal.com/arcade/intro/level-3/JKKuHJknZNj4YGL32
 *  @exports commonCharacterCount
 *  @module src/commonCharacterCount
 *  @function commonCharacterCount
 *  @param {string} s1 - string of lower cased characters
 *  @param {string} s2 - string of lower cased characters
 */
export default function commonCharacterCount(s1, s2) {
  let count = 0;
  let common = new Set(s1 + s2);
  /**
   *  @method counter
   *  @param {string} str - lowercased string
   *  @returns {Object.<string, number>} {key:/[a-z]/, value:/[0-9]/}
   */
  const counter = (str) =>
    [...str].reduce((obj, item) => {
      obj[item] = (obj[item] || 0) + 1;
      return obj;
    }, {});
  let first = counter(s1),
    second = counter(s2);

  return common;
  console.log(first);
  console.log(second);
}
console.log(commonCharacterCount("aabcc", "adcaa"));
