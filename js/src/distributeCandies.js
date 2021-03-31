export default function distributeCandies(candyType) {
  let maximum = candyType.length / 2;
  let count = new Set(candyType);
  if (maximum < count.size) {
    return count.size;
  } else {
    return maximum;
  }
}

console.log(distributeCandies([1, 1, 2, 2, 3, 3]));
