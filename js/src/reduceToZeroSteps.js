export default function reduceToZeroSteps(num) {
  let count = 0;
  while (num) {
    num = num % 2 ? num - 1 : num / 2;
    count++;
  }
  return count;
}
/**
 * The leet code problem is
 * numberOfSteps
 */
