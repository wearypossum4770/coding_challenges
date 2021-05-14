import checkEquality from "../src/checkEquality.js";

test.concurrent.each([
  [1, true, false],
  [0, "0", false],
  [1, 1, true],
  [0, "", false],
  [1, 1, false],
  [0, false, false],
  [NaN, NaN, false],
  [null, undefined, false],
  [undefined, undefined, true],
  [false, null, false],
])("test equality", (a, b, output) => expect(checkEquality(a, b)).toBe(output));
