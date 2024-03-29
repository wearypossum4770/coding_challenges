/**
 * https://app.codesignal.com/standardized-test/GgvonNCksGzjey46c/task/Cw2FtzNhZBqgxFNhr
 * @param {*} a
 * @param {*} l
 * @param {*} r
 * @returns
 */
export default function boundedRatio(a, l, r) {
  let target = Array.from(a.length);
  for (let i = 0; i < a.length; i++) {
    let x = a[i] / (i + 1);
    target[i] = Number.isInteger(x) && x >= l && x <= r;
  }
  return target;
}
console.log(boundedRatio([8, 5, 6, 16, 5], 1, 3));

let tests = [
  [[8, 5, 6, 16, 5], 1, 3, [false, false, true, false, true]],
  [[100], 1, 1000, [true]],
  [[1, 2, 3, 4, 5], 1, 1, [true, true, true, true, true]],
  [[1, 2, 3, 4, 5], 2, 10000, [false, false, false, false, false]],
  [[1000000, 20000], 10000, 10000, [false, true]],
  [
    [65, 46, 60, 164, 243, 228, 231, 298, 231, 211],
    20,
    50,
    [false, true, true, true, false, true, true, false, false, false],
  ],
  [
    [643, 531, 504, 224, 415, 360, 364, 84, 212, 587],
    70,
    80,
    [false, false, false, false, false, false, false, false, false, false],
  ],
  [
    [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
      22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
      40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    ],
    1,
    10000,
    [
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
      true,
    ],
  ],
];
