export default function getOddNumbers(array) {
  let oddNumbers = [];
  for (let index = 0; index < array.length; index++) {
    if (array[index] % 2 != 0) {
      oddNumbers.push(array[index]);
    }
  }
  return oddNumbers;
}
