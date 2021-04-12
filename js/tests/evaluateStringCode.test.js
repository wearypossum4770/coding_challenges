import evaluateStringCode from "../src/evaluateStringCode.js";

test.each([
  ["1+2", 3],
  ["6/(9-7)", 3],
  ["3+2-4", 1],
  ["3*4+1", 13],
  ["5*8-4*9", 4],
  ["3**7", 2187],
  ["(6**3)+3", 219],
])("", (testInput, testOutput) => {
  let func = evaluateStringCode(testInput);
  expect(func).toEqual(testOutput);
});
