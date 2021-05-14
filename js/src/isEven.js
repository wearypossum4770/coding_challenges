// Test.assertEquals(isEven(2), true)
// Test.assertEquals(isEven(3), false)
// Test.assertEquals(isEven(10), true)
// Test.assertEquals(isEven(31), false)
// Test.assertEquals(isEven(666), true)
// Test.assertEquals(isEven(777), false)
// Test.assertEquals(isEven(3482034), true)
// Test.assertEquals(isEven(3482035), false)
/**
 * @copyright https://edabit.com/challenge/nEdLGbAZQ5LaiumP6
 * @param {number} num
 * @returns
 */
export default function isEven(num) {
  return num % 2 === 0;
}
