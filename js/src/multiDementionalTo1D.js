export default function arrayConcatenation(...args) {
  return args.flat(Infinity);
}

console.log(
  arrayConcatenation([
    [1, 2, 3],
    [4, 5],
    [6, 7],
  ])
);
