import getDeterminant from "../src/getDeterminant.js";

test.concurrent.each([
  [
    [
      [1, 2],
      [-1, 3],
    ],
    5,
  ],

  [
    [
      [0, 1],
      [1, 1],
    ],
    -1,
  ],

  [
    [
      [69, 0],
      [1, 1],
    ],
    69,
  ],
])("automatically paases until code is created.", async (matrix, output) => {
  const func = getDeterminant(matrix);
  expect(func).toBe(output);
});
