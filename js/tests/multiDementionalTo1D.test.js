describe("multi deminstional array concatenation", () => {
  test.each([
    [
      [
        [1, 2, 3],
        [4, 5],
        [6, 7],
      ],
      [1, 2, 3, 4, 5, 6, 7],
    ],
    [
      [[1], [2], [3], [4], [5], [6], [7]],
      [1, 2, 3, 4, 5, 6, 7],
    ],
    [
      [
        [1, 2],
        [3, 4],
      ],
      [1, 2, 3, 4],
    ],
    [[[4, 4, 4, 4, 4]], [4, 4, 4, 4, 4]],
    [
      [["a"], ["b", "c"]],
      ["a", "b", "c"],
    ],
  ])("should calculate losses", (testInput, testOutput) => {
    let func = calculateLosses(testInput);
    expect(func).toBe(testOutput);
  });
});
