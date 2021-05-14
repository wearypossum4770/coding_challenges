// Test.assertSimilar(ascDesNone([4, 3, 2, 1], "Asc"), [1, 2, 3, 4])
// Test.assertSimilar(ascDesNone([7, 8, 11, 66], "Des"), [66, 11, 8, 7])
// Test.assertSimilar(ascDesNone([1, 2, 3, 4], "None"),[1, 2, 3, 4])
// Test.assertSimilar(ascDesNone([1, 0, 1, 0], "Asc"), [0, 0, 1, 1])
// Test.assertSimilar(ascDesNone([1, 2, 2, 2, 2, 2, 2], "Des"), [2, 2, 2, 2, 2, 2, 1])
// Test.assertSimilar(ascDesNone([9, 7, 43, 11, 16, 111, 19], "Asc"), [7, 9, 11, 16, 19, 43, 111])
/**
 * @copyright https://edabit.com/challenge/vsoQiYJXRANcYrX3Z
 * @param {Array.<number>} arr - potentially unsorted array
 * @param {string} method - sorting method to use on array.
 * @return  {Array.<number>}
 */
export default function ascendingDescendingNone(arr, str) {
  switch (str) {
    default:
      return arr;
    case "Asc":
      return arr.sort((a, b) => a - b);
    case "Des":
      return arr.sort((a, b) => a - b).reverse();
  }
}
console.log(ascendingDescendingNone([4, 3, 2, 1], "Asc") === [1, 2, 3, 4]);
