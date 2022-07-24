export default function numOfDigits(num) {
  return num ? ~~Math.log10(Math.abs(num)) + 1 : 1;
}
