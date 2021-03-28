import findPerimeter from "../src/findPerimeter.js";
test.each([
  [[6, 7], 26],
  [[20, 10], 60],
  [[2, 9], 22],
])("returns the perimeter of a rectangle", (testInput, testOutput) => {
  let func = findPerimeter(testInput);
  expect(func).toBe(testOutput);
});
