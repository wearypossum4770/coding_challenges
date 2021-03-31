import convertBooleanToString from "../src/convertBooleanToString.js";

test.each([
  [true, "true"],
  [false, "false"],
])("correctly converts bool to str", (testInput, testOutput) => {
  let func = convertBooleanToString(testInput);
  expect(func).toEqual(testOutput);
});
