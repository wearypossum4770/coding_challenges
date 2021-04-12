export default function arrayOfMultiples(num, length) {
  let array = [];
  for (let i = 1; i < length + 1; i++) {
    array.push(num * i);
  }
  return array;
}
