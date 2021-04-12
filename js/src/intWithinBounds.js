export default function intWithinBounds(num, lower, upper) {
  if (Number.isInteger(num)) {
    return num > lower && num < upper;
  }
  return false;
}
