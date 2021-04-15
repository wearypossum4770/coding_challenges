//
// https://dev.to/seanpgallivan/solution-minimum-operations-to-make-array-equal-4f2e
//
export default function minOperations(n) {
  if (n % 2 === 0) {
    return Math.pow(n, 2) / 4;
  } else {
    return (Math.pow(n, 2) - 1) / 4;
  }
}
