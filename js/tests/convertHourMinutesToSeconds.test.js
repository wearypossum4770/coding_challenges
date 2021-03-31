import convertHourMinutesToSeconds from "../src/convertHourMinutesToSeconds.js";
test.each([
  [[1, 0], 3600],
  [[1, 3], 3780],
  [[0, 30], 1800],
])("should make conversion", (tesetInput1, testInput2, testOutput) => {
  let func = convertHourMinutesToSeconds(tesetInput1, testInput2);
  expect(func).toBe(testOutput);
});
