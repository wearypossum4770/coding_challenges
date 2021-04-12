export default function almostIncreasingSequence(sequence) {
  let target = 0;
  let array = [...sequence].sort((a,b)=>a-b);
  for (let index = 0; index < sequence.length; index++) {
  }
console.log(target)
  return array;
}

console.log(almostIncreasingSequence([1, 3, 2, 1]));
console.log(almostIncreasingSequence([1, 3, 2]));
